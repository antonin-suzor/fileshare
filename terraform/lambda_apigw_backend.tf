################################################################################
###### LAMBDA AND APIGATEWAY FOR BACKEND
################################################################################

########################################
### LAMBDA CONFIGURATION
########################################

resource "aws_lambda_function" "lambda_backend" {
  function_name    = "fileshare-backend"
  architectures    = ["arm64"]
  handler          = "bootstrap"
  runtime          = "provided.al2023"
  package_type     = "Zip"
  role             = aws_iam_role.lambda_exec.arn
  filename         = "${path.module}/${var.path_to_backend_zip}"
  source_code_hash = filebase64sha256("${path.module}/${var.path_to_backend_zip}")

  replace_security_groups_on_destroy = true

  vpc_config {
    subnet_ids         = [aws_subnet.private_1.id, aws_subnet.private_2.id]
    security_group_ids = [aws_security_group.lambda_sg.id]
  }

  timeout     = 15
  description = "Backend for ${var.custom_subdomain}.${var.cloudflare_zone_name}"

  environment {
    variables = {
      RUST_BACKTRACE = 1
      RUST_LOG       = "debug"
      JWT_SECRET     = var.jwt_secret
      WEB_HOST       = "https://${var.custom_subdomain}.${var.cloudflare_zone_name}"

      DATABASE_URL = "postgres://${var.rds_username}:${var.rds_password}@${aws_db_instance.postgres.address}:${aws_db_instance.postgres.port}/${var.rds_dtbsname}?sslmode=verify-full&sslrootcert=/var/task/certs/eu-north-1-bundle.pem"

      S3_URL                = "https://s3.eu-north-1.amazonaws.com"
      S3_ACCESS_KEY_ID      = var.s3_access_key_id
      S3_SECRET_ACCESS_KEY  = var.s3_secret_access_key
      S3_REGION             = "eu-north-1"
      S3_PATH_STYLE_BUCKETS = "true"
      S3_BUCKET_NAME        = var.s3_usercontent_bucket_name

      MAIL_USER     = var.mail_user
      MAIL_PASSWORD = var.mail_password
      MAIL_FROM     = var.mail_from

      DISCORD_WEBHOOK_URL = var.discord_webhook_url
    }
  }
}

########################################
### LAMBDA PERMISSIONS
########################################

data "aws_iam_policy_document" "lambda_poldoc" {
  statement {
    actions = [
      "sts:AssumeRole"
    ]
    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
  }
}

resource "aws_iam_role" "lambda_exec" {
  name = "lambda-exec-role"

  assume_role_policy = data.aws_iam_policy_document.lambda_poldoc.json
}

resource "aws_iam_role_policy_attachment" "lambda_iam_basic_policy" {
  role       = aws_iam_role.lambda_exec.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

resource "aws_iam_role_policy_attachment" "lambda_iam_vpc_policy" {
  role       = aws_iam_role.lambda_exec.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
}

########################################
### APIGATEWAY CONNECTION TO LAMBDA
########################################

resource "aws_apigatewayv2_api" "http_api" {
  name          = "fileshare-apigw"
  protocol_type = "HTTP"
}

resource "aws_apigatewayv2_stage" "apigw_stage" {
  api_id      = aws_apigatewayv2_api.http_api.id
  name        = "$default"
  auto_deploy = true
}

resource "aws_apigatewayv2_integration" "lambda_integration" {
  api_id                 = aws_apigatewayv2_api.http_api.id
  integration_type       = "AWS_PROXY"
  integration_uri        = aws_lambda_function.lambda_backend.invoke_arn
  payload_format_version = "1.0"
}

resource "aws_apigatewayv2_route" "api_route" {
  api_id    = aws_apigatewayv2_api.http_api.id
  route_key = "ANY /api/{proxy+}"
  target    = "integrations/${aws_apigatewayv2_integration.lambda_integration.id}"
}

resource "aws_lambda_permission" "apigw" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.lambda_backend.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.http_api.execution_arn}/*"
}
