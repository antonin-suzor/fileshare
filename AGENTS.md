# AGENTS.md

## Project overview

This is a website to share files. The users sign up, upload their files, then get a link to share them and/or download them.

Languages/frameworks:
- database: postgres
- file storage: s3
- backend: rust + axum + sqlx (the backend is deployed on AWS Lambda)
- frontend: typescript + sveltekit + tailwindcss + daisyui (all pages all pre-rendered when building)
- deployment: terraform + aws

## Repository architecture and code guidelines

Do not use emojis anywhere unless explicitly asked to.

### Backend

The backend code is located in `backend`.
It is Rust code, but it is architectured like a Java backend:
- The `controllers` module contains controllers: their purpose is to receive the requests, perform checks for the request validity, and let services run application logic. Controllers should not handle entities, they should only handle DTOs.
- The `dtos` module contains DTOs (Data Transfer Object): they are simply structs that contain data and model the body of http requests/responses.
- The `entities` module contains entities: they are simply structs that contain data and model the tables of the postgres database.
- The `repositories` module contains repositories: their purpose is to manipulate the data storage at the behest of services, handling SQL, caching, and data integrity. Repositories should not handle DTOs, they should only handle entities.
- The `services` module contains services: their purpose is to handle the core application logic.

Every function inside DTOs/controllers/services/repositories/entities:
- should be inside an impl block for the corresponding struct, even if the struct does not hold any data.
- should have as its first argument the db pool, unless the function does not use it of course.

Naming convention for endpoints: `<http verb>_<api route>` all lowercase where url slashes are replaced with underscores and path parameters are included as their name.
When writing the endpoint strings, use a bracket notation (like this: `/api/some-route/{path_param}`), not a `:` notation.
Any application logic or integration with third-party applications should be included in a service.
If a relevant service does not exist, it should be created. There should be one service per third-party integration, as well as services that handle operations on application objects.
When you bubble an error with `anyhow`, you should add context, formatted like this: "Failed to ...".
Never use `.unwrap()` on anything, always use `.expect("... should ...")`.

Migrations are located in `backend/migrations`. They can be created by running `sqlx migrate add <migration_name>` inside the `backend` folder.
They will be automatically applied when the backend starts, or you can use the sqlx cli if you prefer to.

If you need to add a crate, explain why and what it does.
If you need to add environment variables, you should:
- Add them in the .env file
- Add them in the deployment in `terraform/lambda_apigw_backend.tf`
- Create appropriate terraform variables in `terraform/variables.tf`
- Create appropriate GitHub Action variables in the various workflow files that might need it.

### Frontend

The frontend code is located in `frontend`.
Its code architecture follows SvelteKit use.

All routes are located in `frontend/src/routes`.
All static assets are located in `frontend/src/static`.
All type declarations (such as enums and interfaces to describe the data for type-safety) should be located in `frontend/src/lib/types.ts`.
Components should be located in `frontend/src/lib/components`.
Helpers to interact with the backend should be located in `frontend/src/lib/api`.

Do note that this project uses Svelte 5:
- use runes (such as `$state()`) to handle reactivity
- do not import from `$app/stores` as it is deprecated, use `$app/state` which is the new version
- do not use global objects things like `window.location` when you could use sveltekit-provided functions (which is very often the case)
- do not use `$effect`, use `onMount`, and always keep a single `onMount` function per page/component, and place it at the end of the script block

Other guidelines:
- when doing null-coalescing, use the `??` operator, not the `||` operator
- always prefer using html, daisyui, rather than javascript
- do not write any css, always use daisyui or tailwind

This is very important for code quality.

If you need to add a package, explain why and what it does.

### CI/CD

The CI/CD uses GitHub Actions, and is located in `.github/workflows/cicd.yaml`.
The AWS deployment itself is managed via terraform files in `terraform`.
Unless explicitly asked to or necessary, you should not edit those. If you notice something that could change, seek confirmation first.

## Build and test commands

### Backend

Sanity checks:
- code compiles: `cargo check`
- code is idiomatic rust: `cargo clippy`
- code is well-formatted: `cargo fmt` (running it will automatically format the code)

Tests:
- integration tests: `cargo test --lib` (this needs the docker-compose to be up)

Run the code: `bacon dev` (will start a dev server that will reload on file change, there is a proxy for both backend/frontend on localhost:8000)

### Frontend

Sanity checks:
- code compiles: `bun --bun run check`
- code is well-formatted: `bun --bun run format` (running it will automatically format the code)

There are no unit/integration tests in the frontend as of now.

Run the code: `bun --bun run dev` (will start a dev server that will reload on file change, there is a proxy for both backend/frontend on localhost:8000)

