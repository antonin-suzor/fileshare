################################################################################
###### CONFIGURATION FOR CLOUDFLARE DNS (CNAME RECORD TO CLOUDFRONT)
################################################################################

resource "cloudflare_dns_record" "dns_cname_cloudfront" {
  zone_id = var.cloudflare_zone_id
  name    = var.custom_subdomain
  content = aws_cloudfront_distribution.clf_distrib.domain_name
  type    = "CNAME"
  ttl     = 60
  proxied = var.cloudflare_record_proxied
}
