################################################################################
###### NETWORK OBJECTS FOR AWS TO ALLOW TRAFFIC BETWEEN APP COMPONENTS
################################################################################

resource "aws_security_group" "rds_sg" {
  name   = "fileshare-rds-sg"
  vpc_id = aws_vpc.main.id

  description = "Security group for RDS - only allow Lambda SG to connect"

  lifecycle {
    create_before_destroy = true
  }

  tags = { Name = "fileshare-rds-sg" }
}

resource "aws_security_group" "lambda_sg" {
  name   = "fileshare-lambda-sg"
  vpc_id = aws_vpc.main.id

  description = "Security group for Lambda functions that need access to RDS and internet"

  lifecycle {
    create_before_destroy = true
  }

  tags = { Name = "fileshare-lambda-sg" }
}

resource "aws_security_group_rule" "rds_allow_from_lambda" {
  type                     = "ingress"
  from_port                = 5432
  to_port                  = 5432
  protocol                 = "tcp"
  security_group_id        = aws_security_group.rds_sg.id
  source_security_group_id = aws_security_group.lambda_sg.id
}

resource "aws_security_group_rule" "lambda_allow_all_egress" {
  type              = "egress"
  from_port         = 0
  to_port           = 0
  protocol          = "-1"
  cidr_blocks       = ["0.0.0.0/0"]
  ipv6_cidr_blocks  = ["::/0"]
  security_group_id = aws_security_group.lambda_sg.id
}

resource "aws_security_group_rule" "rds_allow_all_egress" {
  type              = "egress"
  from_port         = 0
  to_port           = 0
  protocol          = "-1"
  cidr_blocks       = ["0.0.0.0/0"]
  ipv6_cidr_blocks  = ["::/0"]
  security_group_id = aws_security_group.rds_sg.id
}
