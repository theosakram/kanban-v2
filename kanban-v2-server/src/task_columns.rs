use std::collections::HashMap;

use actix_web::{
    error, get,
    web::{Data, Json, Path},
    Error,
};
use serde::{Deserialize, Serialize};

use sqlx::FromRow;

use crate::tasks::{AppState, Task};

#[derive(Serialize, Deserialize, FromRow, Debug)]
struct TaskColumn {
    id: i32,
    name: String,
}

#[get("/{board_id}")]
async fn get_columns_by_board_id(
    path: Path<i32>,
    state: Data<AppState>,
) -> Result<Json<Vec<TaskColumn>>, Error> {
    let task_columns = sqlx::query_as("SELECT * FROM task_columns WHERE board_id = $1")
        .bind(*path)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(task_columns))
}
