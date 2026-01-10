use axum::{
    routing::get,
    Router,
    extract::{Json, Extension},
    http::{StatusCode, Method},
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use sqlx::Row;
use std::sync::Arc;

pub mod todo_list_dao;

#[derive(Serialize)]
pub struct Message {
    pub text: String,
}

#[derive(Deserialize)]
pub struct CreateTodo{
    pub title: String,
    pub priority: Option<u8>,
}

#[derive(Serialize, Debug)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub priority: u8,
    pub completed: bool,
}
pub fn build_app(db: Arc<todo_list_dao::TodoListDao>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/", get(root))
        .route("/api/todos", get(list_todos).post(create_todo))
        .layer(Extension(db))
        .layer(cors)
}

pub async fn root() -> Json<Message> {
    println!("Handling root request");
    Json(Message {
        text: "Welcome to Rust API".to_string(),
    })
}

pub async fn create_todo(Extension(db): Extension<Arc<todo_list_dao::TodoListDao>>, Json(payload): Json<CreateTodo>) -> (StatusCode, Json<Todo>) {
    println!("Creating todo"); 
    let priority = payload.priority.unwrap_or(0u8);
    let new = Todo {
        id: 0,
        title: payload.title,
        priority,
        completed: false,
    };

    match db.save_todo(&new).await {
        Ok(id) => {
            let todo = Todo { id, title: new.title, priority: new.priority, completed: new.completed };
            (StatusCode::CREATED, Json(todo))
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(new)),
    }
}

pub async fn list_todos(Extension(db): Extension<Arc<todo_list_dao::TodoListDao>>) -> Json<Vec<Todo>> {
    println!("Listing todos...");
    let mut todos: Vec<Todo> = Vec::new();
    if let Ok(rows) = db.query_todos().await {
        for row in rows {
            let id: i32 = row.get("id");
            let title: String = row.get("title");
            let priority: i32 = row.get("priority");
            let completed: bool = row.get("completed");
            todos.push(Todo {
                id: id as u32,
                title,
                priority: priority as u8,
                completed,
            });
        }
    }
    Json(todos)
}