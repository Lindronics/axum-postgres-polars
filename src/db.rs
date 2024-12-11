use sqlx::PgPool;

pub use sqlx::postgres::PgConnectOptions as Config;

use crate::model;

#[derive(Clone)]
/// Wraps a [PgPool]. Since [PgPool] wraps an [Arc], we can clone it cheaply.
pub struct DbClient {
    pub pool: PgPool,
}

impl DbClient {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        Ok(Self {
            pool: PgPool::connect_with(config).await?,
        })
    }

    pub async fn put_boat(&self, boat: &model::Boat) -> Result<(), model::Error> {
        sqlx::query!(
            r#"
            INSERT INTO boats (name, length_ft, rig)
            VALUES ($1, $2, $3)
            ON CONFLICT (name) DO NOTHING
            "#,
            boat.name,
            boat.length_ft,
            boat.rig,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_all_boats(&self) -> Result<Vec<model::Boat>, model::Error> {
        let res = sqlx::query_as!(
            model::Boat,
            r#"
            SELECT
                name,
                length_ft as "length_ft",
                rig
            FROM boats
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(res)
    }
}
