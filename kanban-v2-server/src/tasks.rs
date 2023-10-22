use actix_web::{
    error, get, post,
    web::{Data, Json, Path},
    Error, Result,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::rbac::{authorize, Role, User, UserAction};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Task {
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

#[get("/{task_columns_id}")]
async fn get_tasks_by_columns_id(
    path: Path<i32>,
    state: Data<AppState>,
) -> Result<Json<Vec<Task>>, Error> {
    let user = User {
        username: "Hehe".to_string(),
        role: Role::Collaborator,
    };

    match authorize(&user, &UserAction::ReadTask) {
        Ok(_) => {
            let tasks = sqlx::query_as("SELECT * FROM tasks WHERE column_id = $1")
                .bind(*path)
                .fetch_all(&state.pool)
                .await
                .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

            return Ok(Json(tasks));
        }
        Err(e) => return Err(error::ErrorUnauthorized(e)),
    }
}

#[post("")]
async fn add_task(task: Json<TaskNew>, state: Data<AppState>) -> Result<Json<Task>, Error> {
    let user = User {
        username: "Hehe".to_string(),
        role: Role::Creator,
    };

    match authorize(&user, &UserAction::CreateTask) {
        Ok(_) => {
            let task = sqlx::query_as("INSERT INTO tasks (serial, title, description) VALUES ($1, $2, $3) RETURNING id, serial, title, description")
            .bind(&task.serial)
            .bind(&task.title)
            .bind(&task.description)
            .fetch_one(&state.pool)
            .await
            .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

            return Ok(Json(task));
        }
        Err(e) => return Err(error::ErrorUnauthorized(e)),
    }
}
