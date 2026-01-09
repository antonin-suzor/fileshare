################################################################################
###### VPC AND NETWORKING FOR INTERNET ACCESS
################################################################################

########################################
### VPC
########################################

resource "aws_vpc" "main" {
  cidr_block           = "10.0.0.0/16"
  enable_dns_hostnames = true
  enable_dns_support   = true

  tags = {
    Name = "fileshare-vpc"
  }
}

########################################
### PUBLIC SUBNET (FOR NAT GATEWAY)
########################################

resource "aws_subnet" "public" {
  vpc_id                  = aws_vpc.main.id
  cidr_block              = "10.0.1.0/24"
  availability_zone       = data.aws_availability_zones.available.names[0]
  map_public_ip_on_launch = true

  tags = {
    Name = "fileshare-public-subnet"
  }
}

########################################
### PRIVATE SUBNETS (FOR LAMBDA AND RDS)
########################################

resource "aws_subnet" "private_1" {
  vpc_id            = aws_vpc.main.id
  cidr_block        = "10.0.2.0/24"
  availability_zone = data.aws_availability_zones.available.names[0]

  tags = {
    Name = "fileshare-private-subnet-1"
  }
}

resource "aws_subnet" "private_2" {
  vpc_id            = aws_vpc.main.id
  cidr_block        = "10.0.3.0/24"
  availability_zone = data.aws_availability_zones.available.names[1]

  tags = {
    Name = "fileshare-private-subnet-2"
  }
}

########################################
### INTERNET GATEWAY
########################################

resource "aws_internet_gateway" "main" {
  vpc_id = aws_vpc.main.id

  tags = {
    Name = "fileshare-igw"
  }
}

########################################
### ELASTIC IP FOR NAT GATEWAY
########################################

resource "aws_eip" "nat" {
  domain = "vpc"

  depends_on = [aws_internet_gateway.main]

  tags = {
    Name = "fileshare-nat-eip"
  }
}

########################################
### NAT GATEWAY (IN PUBLIC SUBNET)
########################################

resource "aws_nat_gateway" "main" {
  subnet_id     = aws_subnet.public.id
  allocation_id = aws_eip.nat.id

  tags = {
    Name = "fileshare-nat"
  }

  depends_on = [aws_internet_gateway.main]
}

########################################
### ROUTE TABLES
########################################

# Public route table (routes to Internet Gateway)
resource "aws_route_table" "public" {
  vpc_id = aws_vpc.main.id

  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.main.id
  }

  tags = {
    Name = "fileshare-public-rt"
  }
}

resource "aws_route_table_association" "public" {
  subnet_id      = aws_subnet.public.id
  route_table_id = aws_route_table.public.id
}

# Private route table (routes to NAT Gateway for internet access)
resource "aws_route_table" "private" {
  vpc_id = aws_vpc.main.id

  route {
    cidr_block     = "0.0.0.0/0"
    nat_gateway_id = aws_nat_gateway.main.id
  }

  tags = {
    Name = "fileshare-private-rt"
  }
}

resource "aws_route_table_association" "private_1" {
  subnet_id      = aws_subnet.private_1.id
  route_table_id = aws_route_table.private.id
}

resource "aws_route_table_association" "private_2" {
  subnet_id      = aws_subnet.private_2.id
  route_table_id = aws_route_table.private.id
}

########################################
### DATA SOURCE FOR AVAILABILITY ZONES
########################################

data "aws_availability_zones" "available" {
  state = "available"
}
