use polars::{io::SerReader, prelude::CsvReader};
use sqlx::{postgres::PgPoolCopyExt, PgPool};

use futures::stream::StreamExt;
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

    pub async fn print_all_boats(&self) -> Result<String, model::Error> {
        let output_stream = self
            .pool
            .copy_out_raw(
                r#"
                COPY (
                    SELECT name, length_ft, rig 
                    FROM boats
                ) 
                TO STDOUT CSV HEADER"#,
            )
            .await?;

        // Looks like polars doesn't support async CSV readers from what I can see,
        // so we'll have to read the stream of chunks into memory before we can parse it.
        //
        // TODO implement an async CSV reader.
        //
        // Also, we could use Postgres' binary format directly if we implement a custom parser.
        let csv = output_stream
            .filter_map(|chunk| std::future::ready(chunk.ok()))
            .flat_map(futures::stream::iter)
            .collect::<Vec<_>>()
            .await;

        let df = CsvReader::new(std::io::Cursor::new(csv)).finish()?;

        Ok(df.to_string())
    }
}
