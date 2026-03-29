# RESTful Rust CRUD API
[![CI within Docker](https://github.com/ac5307/rest-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/ac5307/rest-rs/actions/workflows/ci.yml)

A CRUD API built with **Rust**, **Axum**, **SQLx**, **Postgres**, and a REST architectural style.

## Tech Stack
- **Language:** Rust
- **Framework:** [Axum](https://github.com/tokio-rs/axum) (0.8)
- **SQL Toolkit:** [SQLx](https://github.com/launchbadge/sqlx)
- **Database:** PostgreSQL
- **Containerization:** Docker & Docker Compose
- **CI:** GitHub Actions

## Project Structure

The main application is broken up into two major layers: **DB** & **API**.

### DB
The DB layer allows communication with the Postgres database. The main feature is
an utility created in order to make querying the database much cleaner. Normally, when using
prepared statements with SQLx, one would have to bind each argument individually.
However, the utility creates a layer that does that for you.

Definition of the utility: [utils](https://github.com/ac5307/rest-rs/blob/main/src/db/utils.rs)

Usage of the utility: [rest](https://github.com/ac5307/rest-rs/blob/main/src/db/rest.rs)

### API
The API layer handles HTTP requests and exchanges data in JSON. This layer defines the URIs
for the REST resources and uses the resource-methods defined in the DB layer. In order to
perform CRUD (Create, Read, Update, Delete) operations, the resource-methods are mapped to
standard HTTP-methods: POST, GET, PUT, DELETE. Then, all of the endpoints are unified
together for Axum to serve.
