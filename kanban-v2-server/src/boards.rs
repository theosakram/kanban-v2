use actix_web::{
    error, get,
    web::{Data, Json, Path},
    Error, Result,
};
use serde::{Deserialize, Serialize};

use sqlx::FromRow;

use crate::tasks::AppState;

#[derive(Serialize, Deserialize, FromRow, Debug)]
struct Board {
    id: i32,
    name: String,
}

#[get("/{id}")]
async fn get_boards_by_user_id(
    path: Path<i32>,
    state: Data<AppState>,
) -> Result<Json<Vec<Board>>, Error> {
    let boards = sqlx::query_as("SELECT * FROM boards WHERE user_id = $1")
        .bind(*path)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(boards))
}
