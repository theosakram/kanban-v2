mod auth;
mod boards;
mod helper;
mod rbac;
mod task_columns;
mod tasks;

use std::env;

use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{get, web::ServiceConfig};
use auth::{login, register};
use boards::get_boards_by_user_id;
use dotenv::dotenv;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};
use task_columns::get_columns_by_board_id;
use tasks::{add_task, get_tasks_by_columns_id, AppState};

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

async fn get_uri() -> String {
    dotenv().ok();
    let db_uri = env::var("DB_URI").expect("Please set DB_URI in .env");

    db_uri
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = get_uri().await.as_str(),
    )]
    pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    pool.execute(include_str!("./schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let state = web::Data::new(AppState { pool });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world)
            .service(
                web::scope("/boards")
                    .wrap(Logger::default())
                    .service(get_boards_by_user_id),
            )
            .service(
                web::scope("/columns")
                    .wrap(Logger::default())
                    .service(get_columns_by_board_id),
            )
            .service(
                web::scope("/tasks")
                    .wrap(Logger::default())
                    .service(get_tasks_by_columns_id)
                    .service(add_task),
            )
            .service(
                web::scope("/auth")
                    .wrap(Logger::default())
                    .service(login)
                    .service(register),
            )
            .app_data(state);
    };

    Ok(config.into())
}
