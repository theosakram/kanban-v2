use crate::{
    helper::{hash_pw, validate_email, verify_pw},
    AppState,
};
use actix_web::{
    error, post,
    web::{Data, Json},
    Error,
};
use serde::{Deserialize, Serialize};
use sqlx::{Error as SQLXError, FromRow};

#[derive(Serialize, Deserialize, FromRow, Debug)]
struct User {
    fullname: String,
    username: Option<String>,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
struct RegisterRequest {
    fullname: String,
    username: Option<String>,
    email: String,
    password: String,
}

#[post("/login")]
async fn login(req: Json<LoginRequest>, state: Data<AppState>) -> Result<Json<User>, Error> {
    match validate_email(&req.email) {
        true => {
            let user: User =
                sqlx::query_as("SELECT id, fullname, username, email FROM users WHERE email = $1")
                    .bind(&req.email)
                    .fetch_one(&state.pool)
                    .await
                    .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

            match verify_pw(&req.password, &user.password) {
                true => Ok(Json(user)),
                false => Err(error::ErrorBadRequest("Wrong password".to_string())),
            }
        }
        false => Err(error::ErrorBadRequest("Wrong email format".to_string())),
    }
}

#[post("/register")]
async fn register(req: Json<RegisterRequest>, state: Data<AppState>) -> Result<Json<User>, Error> {
    match validate_email(&req.email) {
        true => {
            let user: Result<User, SQLXError> =
                sqlx::query_as("SELECT email FROM users WHERE email = $1")
                    .bind(&req.email)
                    .fetch_one(&state.pool)
                    .await;

            match user {
                Ok(_) => Err(error::ErrorBadRequest(
                    "Email already registered".to_string(),
                )),
                Err(_) => {
                    let query = "INSERT INTO users (fullname, username, email, password VALUES ($1, $2, $3, $4) RETURNING id, fullname, username, email";
                    let new_user = sqlx::query_as(query)
                        .bind(&req.fullname)
                        .bind(&req.username)
                        .bind(&req.email)
                        .bind(hash_pw(&req.password))
                        .fetch_one(&state.pool)
                        .await
                        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

                    Ok(Json(new_user))
                }
            }
        }
        false => Err(error::ErrorBadRequest("Wrong email format".to_string())),
    }
}
