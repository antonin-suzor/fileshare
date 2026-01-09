# fileshare

[![CI](https://github.com/antonin-suzor/fileshare/actions/workflows/cicd.yaml/badge.svg?branch=main)](https://github.com/antonin-suzor/fileshare/actions/workflows/cicd.yaml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.txt)

My website for file sharing: https://fileshare.antonin-suzor.com

Uses Postgres+S3 for data storage, Rust+Axum+SQLX for the backend, TS+SvelteKit+TailwindCSS+DaisyUI for the frontend.
Deployed with Terraform on AWS with RDS, S3, Lambda, ApiGateway, CloudFront.
CloudFlare is used for DNS.

## Prerequisites

- `docker`
- `cargo`
- `sqlx-cli`
- `npm`

## How-to

- Setup the env variables:
  - `cd backend ; cp .env.example .env`
  - Edit the discord webhook env variable (set it blank if you don't need it)
  - `cd frontend ; cp .env.example .env`
- Setup the database locally:
  - `docker compose up -d`
  - `cd backend`
  - `sqlx database reset -y` (If you don't put the `-y`, it will ask for confirmation)
- Dev:
  - `docker compose up`
  - `cd backend ; bacon dev`
  - `cd frontend ; npm run dev`
  - After that, you can go to `http://localhost:8000` (a reverse-proxy is set up)
- Create a new migration:
  - `cd backend`
  - `sqlx migrate add <migration_name>` (replace `<migration_name>` with your migration's name)
  - Start editing the new `.sql` file created in `backend/migrations`

## External Documentation

- https://www.cargo-lambda.info/
- https://docs.aws.amazon.com/lambda/latest/dg/lambda-rust.html
- https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples/
- https://crates.io/crates/sqlx
- https://crates.io/crates/sqlx-cli
- https://svelte.dev/docs
