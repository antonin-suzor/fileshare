################################################################################
###### S3 BUCKETS FOR FILE STORAGE
################################################################################

########################################
### FRONTEND FILES BUCKET
########################################

resource "aws_s3_bucket" "frontend_bucket" {
  bucket        = var.s3_frontend_bucket_name
  force_destroy = true
}

module "template_files" {
  # Yes, we have to use a module and not a for/each,
  # because terraform does not provide a built-in way to get a file's mime type.
  source   = "hashicorp/dir/template"
  base_dir = "${path.module}/${var.path_to_frontend_files}"
}

resource "aws_s3_object" "frontend_files" {
  for_each     = module.template_files.files
  bucket       = aws_s3_bucket.frontend_bucket.id
  key          = each.key
  content_type = each.value.content_type
  # The template_files module guarantees that only one of these two attributes
  # will be set for each file, depending on whether it is an in-memory template
  # rendering result or a static file on disk.
  source  = each.value.source_path
  content = each.value.content
  # Unless the bucket has encryption enabled, the ETag of each object is an
  # MD5 hash of that object.
  etag = each.value.digests.md5
}

########################################
### USER-UPLOADED CONTENT BUCKET
########################################

resource "aws_s3_bucket" "usercontent_bucket" {
  bucket        = var.s3_usercontent_bucket_name
  force_destroy = false
}

resource "aws_s3_bucket_cors_configuration" "usercontent_bucket_cors" {
  bucket = aws_s3_bucket.usercontent_bucket.id

  cors_rule {
    allowed_headers = ["*"]
    allowed_methods = ["GET", "HEAD", "PUT"]
    allowed_origins = ["https://${var.custom_subdomain}.${var.cloudflare_zone_name}"]
    expose_headers  = ["ETag"]
  }

  cors_rule {
    allowed_methods = ["GET", "HEAD"]
    allowed_origins = ["*"]
  }
}

