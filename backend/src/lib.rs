use axum::{
    routing::{get, post},
    Router,
    extract::Json,
    http::{StatusCode, Method},
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub mod todo_list_dao;

#[derive(Serialize)]
pub struct Message {
    pub text: String,
}

#[derive(Deserialize)]
pub struct CreateTodo{
    pub name: String,
    pub priority: u8,
}

#[derive(Serialize)]
pub struct Todo {
    pub id: u64,
    pub name: String,
    pub priority: u8,
}

pub fn build_app() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/", get(root))
        .route("/todos", post(create_todo))
        .layer(cors)
}

pub async fn root() -> Json<Message> {
    Json(Message {
        text: "Welcome to Rust API".to_string(),
    })
}

pub async fn create_todo(Json(payload): Json<CreateTodo>) -> (StatusCode, Json<Todo>) {
    let todo = Todo {
        id: 1,
        name: payload.name,
        priority: payload.priority,
    };

    (StatusCode::CREATED, Json(todo))
}

pub async fn create_pool() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut attempts = 0u32;
    let max_attempts = 10u32;
    let mut delay = Duration::from_secs(1);

    loop {
        match PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
        {
            Ok(pool) => return Ok(pool),
            Err(e) => {
                attempts += 1;
                if attempts >= max_attempts {
                    return Err(e);
                }
                eprintln!(
                    "DB connect attempt {}/{} failed: {}. retrying in {}s...",
                    attempts,
                    max_attempts,
                    e,
                    delay.as_secs()
                );
                tokio::time::sleep(delay).await;
                delay = std::cmp::min(delay * 2, Duration::from_secs(10));
            }
        }
    }
}
