################################################################################
###### GENERAL TERRAFORM CONFIGURATION
################################################################################

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 6.15"
    }
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 5.11"
    }
  }
  backend "s3" {
    bucket       = "asuzorepita-terraform-backend"
    key          = "fileshare-asuzor/terraform.tfstate"
    region       = "eu-north-1"
    use_lockfile = true
  }
  required_version = ">= 1.2"
}

provider "aws" {
  region = "eu-north-1"
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}
