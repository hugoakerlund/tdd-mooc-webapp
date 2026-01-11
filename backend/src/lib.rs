use axum::{
    routing::{get, post},
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

#[derive(Deserialize)]
pub struct IdPayload {
    pub id: u32,
}

#[derive(Deserialize)]
pub struct RenamePayload {
    pub id: u32,
    pub new_title: String,
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
        .route("/api/todos/complete", post(toggle_todo_completion))
        .route("/api/todos/delete", post(delete_todo))
        .route("/api/todos/increase_priority", post(increase_todo_priority))
        .route("/api/todos/decrease_priority", post(decrease_todo_priority))
        .route("/api/todos/clear", post(clear_todo_list))
        .route("/api/todos/archive_completed", post(archive_completed_todos))
        .route("/api/todos/rename", post(rename_todo))
        .layer(Extension(db))
        .layer(cors)
}

pub async fn root() -> Json<Message> {
    println!("Handling root request");
    Json(Message {
        text: "Welcome to Rust API".to_string(),
    })
}

pub async fn create_todo(Extension(
    db): Extension<Arc<todo_list_dao::TodoListDao>>,
    Json(payload): Json<CreateTodo>) 
    -> (StatusCode, Json<Todo>) {
    println!("Creating todo"); 
    let priority = payload.priority.unwrap_or(1u8);
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

pub async fn archive_completed_todos(Extension(
    db): Extension<Arc<todo_list_dao::TodoListDao>>) 
    -> (StatusCode, Json<Message>) {
    println!("Archiving completed todos...");

    match db.archive_completed_todos().await {
        Ok(count) => {
            let msg = Message { text: format!("Archived {} completed todo(s)", count) };
            (StatusCode::OK, Json(msg))
        }
        Err(_) => {
            let msg = Message { text: "Failed to archive completed todos".to_string() };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(msg))
        }
    }
}

pub async fn rename_todo(Extension(
    db): Extension<Arc<todo_list_dao::TodoListDao>>, 
    Json(payload): Json<RenamePayload>) 
    -> (StatusCode, Json<Todo>) {
    println!("Renaming todo");
    let id = payload.id as u64;
    let new_title = payload.new_title;

    match db.rename_todo(id, new_title.clone()).await {
        Ok(id_u32) => {
            let todo = Todo { id: id_u32, title: new_title.to_string(), priority: 0, completed: false };
            (StatusCode::ACCEPTED, Json(todo))
        }
        Err(_) => {
            let fallback = Todo { id: payload.id, title: new_title.to_string(), priority: 0, completed: false };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(fallback))
        }
    }
}

pub async fn toggle_todo_completion(Extension(
    db): Extension<Arc<todo_list_dao::TodoListDao>>, 
    Json(payload): Json<IdPayload>) -> (StatusCode, Json<Todo>) {
    println!("Changing todo to completed");

    let id = payload.id as u64;

    match db.toggle_todo_completion(id).await {
        Ok(id_u32) => {
            let todo = Todo { id: id_u32, title: String::new(), priority: 0, completed: true };
            (StatusCode::ACCEPTED, Json(todo))
        }
        Err(_) => {
            let fallback = Todo { id: payload.id, title: String::new(), priority: 0, completed: true };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(fallback))
        }
    }
}

pub async fn delete_todo(Extension(
    db): Extension<Arc<todo_list_dao::TodoListDao>>, 
    Json(payload): Json<IdPayload>) -> (StatusCode, Json<Message>) {
    println!("Deleting todo");

    let id = payload.id as u64;

    match db.delete_todo(id).await {
        Ok(_) => {
            let msg = Message { text: format!("Todo with id {} deleted successfully", payload.id) };
            (StatusCode::OK, Json(msg))
        }
        Err(_) => {
            let msg = Message { text: format!("Failed to delete todo with id {}", payload.id) };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(msg))
        }
    }
}

pub async fn increase_todo_priority(Extension(
    db): Extension<Arc<todo_list_dao::TodoListDao>>, 
    Json(payload): Json<IdPayload>) -> (StatusCode, Json<Message>) {
    println!("Increasing todo priority");

    let id = payload.id as u64;

    match db.increase_todo_priority(id).await {
        Ok(id_u32) => {
            (StatusCode::ACCEPTED, Json(Message { text: format!("Todo with id {} priority increased", id_u32) }))
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Message { text: "Failed to increase todo priority".into() }))
        }
    }
}

pub async fn decrease_todo_priority(Extension(
    db): Extension<Arc<todo_list_dao::TodoListDao>>, 
    Json(payload): Json<IdPayload>) -> (StatusCode, Json<Message>) {
    println!("Increasing todo priority");

    let id = payload.id as u64;

    match db.decrease_todo_priority(id).await {
        Ok(id_u32) => {
            (StatusCode::ACCEPTED, Json(Message { text: format!("Todo with id {} priority decreased", id_u32) }))
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Message { text: "Failed to decrease todo priority".into() }))
        }
    }
}

pub async fn clear_todo_list(Extension(
    db): Extension<Arc<todo_list_dao::TodoListDao>>) 
    -> (StatusCode, Json<Message>) {
    println!("Clearing todo list...");
    match db.truncate_todos_table().await {
        Ok(_) => (StatusCode::OK, Json(Message { text: "All todos have been deleted".to_string() })),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Message { text: "Failed to clear todo list".to_string() })),
    }
}

pub async fn list_todos(Extension(
    db): Extension<Arc<todo_list_dao::TodoListDao>>) 
    -> Json<Vec<Todo>> {
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