################################################################################
###### RDS POSTGRES FOR DATABASE
################################################################################

resource "aws_db_subnet_group" "dbsubnet" {
  name       = "pae-db-subnet-group"
  subnet_ids = data.aws_subnets.default.ids
  tags       = { Name = "fileshare-db-subnet-group" }
}

resource "aws_db_instance" "postgres" {
  identifier                      = "db-fileshare-1"
  engine                          = "postgres"
  instance_class                  = "db.t4g.micro"
  db_name                         = var.rds_dtbsname
  username                        = var.rds_username
  password                        = var.rds_password
  allocated_storage               = 20
  storage_type                    = "gp2"
  db_subnet_group_name            = aws_db_subnet_group.dbsubnet.name
  vpc_security_group_ids          = [aws_security_group.rds_sg.id]
  skip_final_snapshot             = true
  deletion_protection             = false
  backup_retention_period         = 0
  publicly_accessible             = false
  storage_encrypted               = false
  performance_insights_enabled    = false
  enabled_cloudwatch_logs_exports = ["postgresql", "iam-db-auth-error"]
  multi_az                        = false
  tags = {
    Name = "db-fileshare-1"
  }
}
