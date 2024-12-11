# Axum - SQLx - Polars test

View tests for e2e flow.

## Prerequisites

- docker
- sqlx-cli (cargo plugin)

## Building and running in non-offline mode

If `SQLX_OFFLINE=false`, run the following steps to build the project:

```sh
docker compose up -d
cargo install sqlx-cli
cargo sqlx migrate run
cargo run
```

## Running tests

```sh
cargo test
```
