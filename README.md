# About
This project aims to generalize the creation of RESTful web services by using database models to derive a suite of http endpoints that can be exposed with a webserver module like [Actix](https://github.com/actix/actix). It also aims to provide these features as high-level abstractions:

* Authentication
* Authorization
* Cursor based pagination
* Powerful filtering

# Disclaimer
This project is still in active pre-alpha development. Interfaces will undergo breaking changes, and usage docs may not be adequately informative until the project stabilizes.

# Usage:
* Start Postgres container with Docker Compose: `docker-compose up -d database`
* Init Diesel: `diesel setup`
* Create Migrations: `diesel migration generate create_posts`
* Run migrations: `diesel migration run`
* Rollback migrations: `diesel migration redo`
* Run server: `cargo run --bin run_server`
