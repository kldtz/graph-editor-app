# Graph Editor App

[Graph editor](https://github.com/kldtz/graph-editor) based on Colorado Reed's tool for creating directed graphs with a back end written in Rust using a Postgres database.

## Development installation

1. Install Postgres DB and adjust DATABASE_URL environment variable in `.env` file
2. Install Diesel client: `cargo install diesel_cli`
3. Set up DB and run migrations: `diesel database setup`
4. Run the app, watch files and execute on changes: `cargo watch -x run`