use axum::{extract::State, Json};

use crate::{model, AppContext};

pub async fn health_check(State(_state): State<AppContext>) -> &'static str {
    "I'm alive"
}

pub async fn put_boat(
    State(state): State<AppContext>,
    Json(boat): Json<model::Boat>,
) -> Result<Json<model::Boat>, model::Error> {
    state.db.put_boat(&boat).await?;
    Ok(Json(boat))
}

pub async fn get_all_boats(
    State(state): State<AppContext>,
) -> Result<Json<Vec<model::Boat>>, model::Error> {
    let boats = state.db.get_all_boats().await?;
    Ok(Json(boats))
}

pub async fn print_all_boats(State(state): State<AppContext>) -> Result<String, model::Error> {
    let s = state.db.print_all_boats().await?;
    Ok(s)
}
