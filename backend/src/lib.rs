use axum::{
    routing::{get, post},
    Router,
    extract::Json,
    http::{StatusCode, Method},
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

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

#[derive(Serialize, Debug)]
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