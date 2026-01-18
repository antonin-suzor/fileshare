################################################################################
###### EC2 AND APIGATEWAY FOR BACKEND
################################################################################

########################################
### EC2 CONFIGURATION
########################################

resource "aws_instance" "backend" {
  ami                    = "ami-0fa91bc90632c73c9" # Ubuntu Server 24.04
  instance_type          = "t3.micro"
  vpc_security_group_ids = [aws_security_group.ec2_sg.id]
  key_name               = "4eeee"

  root_block_device {
    volume_type           = "gp3"
    volume_size           = 8
    delete_on_termination = true
    encrypted             = false
  }

  tags = {
    Name = "fileshare-backend"
  }

  user_data = templatefile("${path.module}/ec2_userdata.tpl", {
    JWT_SECRET = var.jwt_secret
    WEB_HOST   = "https://${var.custom_subdomain}.${var.cloudflare_zone_name}"

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
  })
}

########################################
### APIGATEWAY CONNECTION TO EC2
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
  integration_type       = "HTTP_PROXY"
  integration_uri        = "http://${aws_instance.backend.public_ip}"
  integration_method     = "ANY"
  connection_type        = "INTERNET"
  payload_format_version = "1.0"
}

resource "aws_apigatewayv2_route" "api_route" {
  api_id    = aws_apigatewayv2_api.http_api.id
  route_key = "ANY /api/{proxy+}"
  target    = "integrations/${aws_apigatewayv2_integration.lambda_integration.id}"
}
