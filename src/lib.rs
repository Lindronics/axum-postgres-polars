use axum::{routing::get, Router};
use config::Config;
use db::DbClient;

pub mod config;
mod db;
pub mod model;
mod routes;

#[derive(Clone)]
pub struct AppContext {
    pub db: DbClient,
}

impl AppContext {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let db = DbClient::new(config.db).await?;
        Ok(Self { db })
    }
}

pub async fn start(config: Config, listener: tokio::net::TcpListener) -> anyhow::Result<()> {
    let context = AppContext::new(config).await?;
    let app = Router::new()
        .route("/up", get(routes::health_check))
        .route("/boats", get(routes::get_all_boats).put(routes::put_boat))
        .with_state(context);
    axum::serve(listener, app).await?;
    Ok(())
}
