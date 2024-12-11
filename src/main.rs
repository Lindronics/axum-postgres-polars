#[tokio::main]
async fn main() {
    let cfg = axum_postgres_polars::config::Config {
        db: axum_postgres_polars::config::Db::new()
            .username("postgres")
            .password("password")
            .host("localhost")
            .port(5432),
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum_postgres_polars::start(cfg, listener).await.unwrap();
}
