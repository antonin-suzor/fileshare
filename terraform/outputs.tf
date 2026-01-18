################################################################################
###### OUTPUTS OF A TERRAFORM APPLY
################################################################################

output "cloudfront_domain_name" {
  description = "Domain name of the CloudFront distribution (used for creating DNS CNAME in Cloudflare)."
  value       = aws_cloudfront_distribution.clf_distrib.domain_name
}

output "domain_name" {
  description = "Public-facing URL-host of the website"
  value       = "${var.custom_subdomain}.${var.cloudflare_zone_name}"
}

output "ec2_public_ip" {
  description = "Public IP of EC2 instance"
  value       = aws_instance.backend.public_ip
}
