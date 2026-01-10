use backend::{root, create_todo, CreateTodo};
use backend::todo_list_dao::TodoListDao;
use axum::extract::Json as AxumJson;
use axum::http::StatusCode;
use std::sync::Once;

static INIT: Once = Once::new();

#[tokio::test]
async fn test_root_returns_welcome_message() {
    let res = root().await;
    assert_eq!(res.0.text, "Welcome to Rust API");
}

#[tokio::test]
async fn test_create_todo_returns_created_todo() {
    let payload = CreateTodo { name: "Test".to_string(), priority: 2 };
    let (status, json) = create_todo(AxumJson(payload)).await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(json.0.id, 1);
    assert_eq!(json.0.name, "Test");
    assert_eq!(json.0.priority, 2);
}

#[tokio::test]
async fn test_create_pool() {
    INIT.call_once(|| {
        tokio::spawn(async {
            let pool_result = backend::create_pool().await;
            assert!(pool_result.is_ok(), "Failed to create database pool");
        });
    });
}

#[tokio::test]
async fn test_create_dao() {
    INIT.call_once(|| {
        tokio::spawn(async {
            let dao: TodoListDao = TodoListDao::new().await.unwrap();
            assert!(dao.is_open(), "Failed to create TodoListDao");
        });
    });
}

#[tokio::test]
async fn test_create_table() {
    INIT.call_once(|| {
        tokio::spawn(async {
            let dao: TodoListDao = TodoListDao::new().await.unwrap();
            let result = dao.create_table().await.unwrap();
            assert_eq!(result, "Database table created successfully");
        });
    });
}

#[tokio::test]
#[ignore]
async fn test_query_todos() {
    INIT.call_once(|| {
        tokio::spawn(async {
            let dao: TodoListDao = TodoListDao::new().await.unwrap();
            let todos = dao.query_todos().await.unwrap();
            assert!(todos.is_empty(), "Expected no todos in the database");
        });
    });
}