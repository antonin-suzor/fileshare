################################################################################
###### TERRAFORM VARIABLES TO CONFIGURE DEPLOYMENT AND SECRETS
################################################################################

########################################
### PATH TO FILES
########################################

variable "path_to_frontend_files" {
  description = "Path to the folder containing compiled frontend files"
  type        = string
  default     = "../frontend/build"
}

variable "path_to_backend_zip" {
  description = "Path to the zip file containing the lambda function"
  type        = string
  default     = "../backend/cargo_lambda_build_result/bootstrap.zip"
}

########################################
### AWS SETTINGS
########################################

variable "s3_frontend_bucket_name" {
  description = "Name of the s3 bucket for the frontend (must be unique across AWS)"
  type        = string
  default     = "fileshare-asuzor-frontend"
}

variable "s3_usercontent_bucket_name" {
  description = "Name of the s3 bucket fro the user-uploaded content (must be unique across AWS)"
  type        = string
  default     = "fileshare-asuzor-usercontent"
}

########################################
### RDS POSTGRES DATABASE SETTINGS
########################################

variable "rds_dtbsname" {
  description = "Database name to connect to rds database"
  type        = string
  sensitive   = true
}

variable "rds_username" {
  description = "Username to connect to rds database"
  type        = string
  sensitive   = true
}

variable "rds_password" {
  description = "Password to connect to rds database"
  type        = string
  sensitive   = true
}

########################################
### BACKEND GENERAL SETTINGS
########################################

variable "jwt_secret" {
  description = "Secret for backend to use jwt tokens"
  type        = string
  sensitive   = true
}

variable "s3_access_key_id" {
  description = "Access key for AWS S3"
  type        = string
  sensitive   = true
}

variable "s3_secret_access_key" {
  description = "Secret for AWS S3 access key"
  type        = string
  sensitive   = true
}

########################################
### BACKEND EMAIL SETTINGS
########################################

variable "mail_from" {
  description = "Email address to send emails from"
  type        = string
  sensitive   = true
}

variable "mail_user" {
  description = "Username to connect to the SMTP server"
  type        = string
  sensitive   = true
}

variable "mail_password" {
  description = "Password to connect to the SMTP server"
  type        = string
  sensitive   = true
}

########################################
### BACKEND DISCORD SETTINGS
########################################

variable "discord_webhook_url" {
  description = "Discord webhook URL for sending notifications"
  type        = string
  sensitive   = true
}

########################################
### CLOUDFLARE DNS SETTINGS
########################################

variable "custom_subdomain" {
  description = "Custom subdomain to use for CloudFront (so, for `fileshare.antonin-suzor.com`, input `fileshare` here)"
  type        = string
  default     = "fileshare"
}

variable "acm_certificate_arn" {
  description = "ARN of ACM certificate issued in us-east-1 to attach to the CloudFront distribution"
  type        = string
  sensitive   = true
}

variable "cloudflare_api_token" {
  description = "Cloudflare API token with zone-dns-edit"
  type        = string
  sensitive   = true
}

variable "cloudflare_zone_id" {
  description = "Cloudflare Zone ID"
  type        = string
  default     = "9f61628c3f73deb195bbe36499d3fd7a"
}

variable "cloudflare_zone_name" {
  description = "Cloudflare Zone name"
  type        = string
  default     = "antonin-suzor.com"
}

variable "cloudflare_record_proxied" {
  description = "Whether Cloudflare should proxy the CNAME record"
  type        = bool
  default     = false
}
