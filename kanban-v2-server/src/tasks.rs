use actix_web::{
    error, get, post,
    web::{Data, Json},
    Result,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Serialize, Deserialize, FromRow, Debug)]
struct Task {
    id: i32,
    serial: Option<String>,
    title: String,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
struct TaskNew {
    serial: Option<String>,
    title: String,
    description: Option<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

#[get("")]
async fn get_tasks(state: Data<AppState>) -> Result<Json<Vec<Task>>> {
    let tasks = sqlx::query_as("SELECT * FROM tasks")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    return Ok(Json(tasks));
}

#[post("")]
async fn add_task(task: Json<TaskNew>, state: Data<AppState>) -> Result<Json<Task>> {
    let task = sqlx::query_as("INSERT INTO tasks (serial, title, description) VALUES ($1, $2, $3) RETURNING id, serial, title, description")
        .bind(&task.serial)
        .bind(&task.title)
        .bind(&task.description)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    return Ok(Json(task));
}
