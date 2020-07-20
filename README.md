
# Usage:
* Start Postgres Container: `docker-compose up -d database`
* Init Diesel: `diesel setup`
* Create Migrations: `diesel migration generate create_posts`
* Run migrations: `diesel migration run`
* Rollback migrations: `diesel migration redo`
