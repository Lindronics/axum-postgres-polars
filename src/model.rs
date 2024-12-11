use axum::response::IntoResponse;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Boat {
    pub name: String,
    pub length_ft: i32,
    pub r#rig: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Db error: {0}")]
    DbError(#[from] sqlx::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            Error::DbError(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                self.to_string(),
            ),
        };
        (status, body).into_response()
    }
}
