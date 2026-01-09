use backend::{root, create_todo, CreateTodo};
use axum::extract::Json as AxumJson;
use axum::http::StatusCode;

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
