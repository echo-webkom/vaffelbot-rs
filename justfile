set dotenv-load

migrate:
    sqlx migrate run --source migrations --database-url "$DATABASE_URL"
