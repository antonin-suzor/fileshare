################################################################################
###### CLOUDFRONT ENDPOINT, PROXY, AND CACHING FOR THE APPLICATION
################################################################################

resource "aws_cloudfront_origin_access_control" "oac" {
  name                              = "gena2-pl3-clf-oac"
  description                       = "Access control for S3"
  origin_access_control_origin_type = "s3"
  signing_behavior                  = "always"
  signing_protocol                  = "sigv4"
}

data "aws_iam_policy_document" "origin_bucket_policy" {
  statement {
    sid    = "AllowCloudFrontServicePrincipalReadWrite"
    effect = "Allow"

    principals {
      type        = "Service"
      identifiers = ["cloudfront.amazonaws.com"]
    }

    actions = [
      "s3:GetObject",
    ]

    resources = [
      "${aws_s3_bucket.frontend_bucket.arn}/*",
    ]

    condition {
      test     = "StringEquals"
      variable = "AWS:SourceArn"
      values   = [aws_cloudfront_distribution.clf_distrib.arn]
    }
  }
}

resource "aws_s3_bucket_policy" "b" {
  bucket = aws_s3_bucket.frontend_bucket.bucket
  policy = data.aws_iam_policy_document.origin_bucket_policy.json
}

data "aws_cloudfront_origin_request_policy" "clf_apigw_origin_request_policy_nohost" {
  name = "Managed-AllViewerExceptHostHeader"
}

data "aws_cloudfront_cache_policy" "clf_apigw_cache_policy_nocache" {
  name = "Managed-CachingDisabled"
}

resource "aws_cloudfront_distribution" "clf_distrib" {
  enabled = true

  origin {
    domain_name = aws_s3_bucket.frontend_bucket.bucket_regional_domain_name
    origin_id   = "s3-frontend"

    origin_access_control_id = aws_cloudfront_origin_access_control.oac.id
  }

  origin {
    domain_name = replace(aws_apigatewayv2_api.http_api.api_endpoint, "https://", "")
    origin_id   = "api-origin"
    custom_origin_config {
      origin_protocol_policy = "https-only"
      http_port              = 80
      https_port             = 443
      origin_ssl_protocols   = ["TLSv1.2"]
    }
  }

  default_cache_behavior {
    target_origin_id       = "s3-frontend"
    viewer_protocol_policy = "redirect-to-https"
    allowed_methods        = ["GET", "HEAD"]
    cached_methods         = ["GET", "HEAD"]

    function_association {
      event_type   = "viewer-request"
      function_arn = aws_cloudfront_function.clf_function_request_handler.arn
    }

    forwarded_values {
      query_string = false
      cookies {
        forward = "none"
      }
    }
  }

  ordered_cache_behavior {
    path_pattern           = "/api/*"
    target_origin_id       = "api-origin"
    viewer_protocol_policy = "redirect-to-https"
    allowed_methods        = ["GET", "HEAD", "OPTIONS", "PUT", "POST", "DELETE", "PATCH"]
    cached_methods         = ["GET", "HEAD", "OPTIONS"]

    origin_request_policy_id = data.aws_cloudfront_origin_request_policy.clf_apigw_origin_request_policy_nohost.id
    cache_policy_id          = data.aws_cloudfront_cache_policy.clf_apigw_cache_policy_nocache.id
  }

  price_class = "PriceClass_100"

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  aliases = ["${var.custom_subdomain}.${var.cloudflare_zone_name}"]
  viewer_certificate {
    acm_certificate_arn            = var.acm_certificate_arn
    ssl_support_method             = "sni-only"
    minimum_protocol_version       = "TLSv1.2_2021"
    cloudfront_default_certificate = false
  }
}

resource "aws_cloudfront_function" "clf_function_request_handler" {
  name    = "fileshare-clfunc"
  runtime = "cloudfront-js-2.0"
  publish = true
  code    = file("${path.module}/cloudfront.js")
}
