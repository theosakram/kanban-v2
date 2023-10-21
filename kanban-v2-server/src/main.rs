mod rbac;
mod tasks;

use std::env;

use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{get, web::ServiceConfig};
use dotenv::dotenv;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};
use tasks::{add_task, get_tasks, AppState};

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
        cfg.service(hello_world).service(
            web::scope("/tasks")
                .wrap(Logger::default())
                .service(get_tasks)
                .service(add_task)
                .app_data(state),
        );
    };

    Ok(config.into())
}
