################################################################################
###### NETWORK OBJECTS FOR AWS TO ALLOW TRAFFIC BETWEEN APP COMPONENTS
################################################################################

data "aws_vpc" "default" {
  default = true
}

data "aws_subnets" "default" {
  filter {
    name   = "vpc-id"
    values = [data.aws_vpc.default.id]
  }
}

resource "aws_security_group" "rds_sg" {
  name   = "fileshare-rds-sg"
  vpc_id = data.aws_vpc.default.id

  description = "Security group for RDS - only allow Lambda SG to connect"

  lifecycle {
    create_before_destroy = true
  }

  tags = { Name = "fileshare-rds-sg" }
}

resource "aws_security_group" "ec2_sg" {
  name   = "fileshare-ec2-sg"
  vpc_id = data.aws_vpc.default.id

  description = "Security group for the EC2 backend that need access to RDS and internet"

  lifecycle {
    create_before_destroy = true
  }

  tags = { Name = "fileshare-ec2-sg" }
}

resource "aws_security_group_rule" "rds_allow_from_ec2" {
  type                     = "ingress"
  from_port                = 5432
  to_port                  = 5432
  protocol                 = "tcp"
  security_group_id        = aws_security_group.rds_sg.id
  source_security_group_id = aws_security_group.ec2_sg.id
}

resource "aws_security_group_rule" "rds_allow_all_egress" {
  type              = "egress"
  from_port         = 0
  to_port           = 0
  protocol          = "-1"
  security_group_id = aws_security_group.rds_sg.id
  cidr_blocks       = ["0.0.0.0/0"]
  ipv6_cidr_blocks  = ["::/0"]
}

resource "aws_security_group_rule" "ec2_allow_ssh_from_all" {
  type              = "ingress"
  from_port         = 22
  to_port           = 22
  protocol          = "tcp"
  security_group_id = aws_security_group.rds_sg.id
  cidr_blocks       = ["0.0.0.0/0"]
  ipv6_cidr_blocks  = ["::/0"]
}

resource "aws_security_group_rule" "ec2_allow_http_from_all" {
  type              = "ingress"
  from_port         = 80
  to_port           = 80
  protocol          = "tcp"
  security_group_id = aws_security_group.rds_sg.id
  cidr_blocks       = ["0.0.0.0/0"]
  ipv6_cidr_blocks  = ["::/0"]
}

resource "aws_security_group_rule" "ec2_allow_all_egress" {
  type              = "egress"
  from_port         = 0
  to_port           = 0
  protocol          = "-1"
  cidr_blocks       = ["0.0.0.0/0"]
  ipv6_cidr_blocks  = ["::/0"]
  security_group_id = aws_security_group.ec2_sg.id
}
