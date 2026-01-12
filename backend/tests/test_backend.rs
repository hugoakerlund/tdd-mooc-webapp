use backend::{CreateTodo, 
              IdPayload, 
              RenamePayload,
              create_todo, 
              archive_completed_todos,
              rename_todo,
              toggle_todo_completion, 
              delete_todo,
              increase_todo_priority,
              decrease_todo_priority,
              clear_todo_list,
              root};
use backend::todo_list_dao::TodoListDao;
use axum::http::StatusCode;
use std::sync::Arc;

#[tokio::test]
async fn test_root_returns_welcome_message() {
    let res = root().await;
    assert_eq!(res.0.text, "Hello from backend!");
}

#[tokio::test]
async fn test_create_todo() {
    let payload = CreateTodo { title: "Test".to_string(), priority: Some(2) };
    let dao = TodoListDao::new().await.unwrap();
    dao.initialize().await;
    let (status, json) = create_todo(axum::Extension(Arc::new(dao)), axum::Json(payload)).await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(json.0.id, 1);
    assert_eq!(json.0.title, "Test");
    assert_eq!(json.0.priority, 2);
    assert_eq!(json.0.completed, false);
}

#[tokio::test]
async fn test_archive_completed_todos() {
    let dao = TodoListDao::new().await.unwrap();
    dao.initialize().await;
    let todo1 = backend::Todo {
        id: 1,
        title: "Completed Todo".to_string(),
        priority: 1,
        completed: true,
    };
    let todo2 = backend::Todo {
        id: 2,
        title: "Incomplete Todo".to_string(),
        priority: 1,
        completed: false,
    };
    dao.save_todo(&todo1).await.unwrap();
    dao.save_todo(&todo2).await.unwrap();

    let (status, json) = archive_completed_todos(axum::Extension(Arc::new(dao))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json.0.text, "Archived 1 completed todo(s)");
}

#[tokio::test]
async fn test_rename_todo() {
    let payload = RenamePayload { id: 1, new_title: "New Title".to_string() };
    let dao = TodoListDao::new().await.unwrap();
    dao.initialize().await;
    let todo = backend::Todo {
        id: 1,
        title: "Old Title".to_string(),
        priority: 1,
        completed: false,
    };
    dao.save_todo(&todo).await.unwrap();
    let new_title = "New Title".to_string();
    let (status, json) = rename_todo(axum::Extension(Arc::new(dao)), axum::Json(payload)).await;
    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(json.0.id, 1);
    assert_eq!(json.0.title, new_title);
}

#[tokio::test]
async fn test_toggle_todo_completion() {
    let dao = TodoListDao::new().await.unwrap();
    dao.initialize().await;
    let (status, json) = toggle_todo_completion(axum::Extension(Arc::new(dao)), axum::Json(IdPayload { id: 1 })).await;
    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(json.0.id, 1);
    assert_eq!(json.0.priority, 0);
    assert_eq!(json.0.completed, true);
}

#[tokio::test]
async fn test_increase_todo_priority() {
    let dao = TodoListDao::new().await.unwrap();
    dao.initialize().await;
    let todo = backend::Todo {
        id: 1,
        title: "Test Priority".to_string(),
        priority: 1,
        completed: false,
    };
    dao.save_todo(&todo).await.unwrap();
    let (status, json) = increase_todo_priority(axum::Extension(Arc::new(dao)), axum::Json(IdPayload { id: 1 })).await;
    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(json.0.text, "Todo with id 1 priority increased");
}

#[tokio::test]
async fn test_decrease_todo_priority() {
    let dao = TodoListDao::new().await.unwrap();
    dao.initialize().await;
    let todo = backend::Todo {
        id: 1,
        title: "Test Priority".to_string(),
        priority: 1,
        completed: false,
    };
    dao.save_todo(&todo).await.unwrap();
    let (status, json) = decrease_todo_priority(axum::Extension(Arc::new(dao)), axum::Json(IdPayload { id: 1 })).await;
    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(json.0.text, "Todo with id 1 priority decreased");
}

#[tokio::test]
async fn test_clear_todo_list() {
    let dao = TodoListDao::new().await.unwrap();
    dao.initialize().await;
    let todo = backend::Todo {
        id: 1,
        title: "Test Truncate".to_string(),
        priority: 1,
        completed: false,
    };
    dao.save_todo(&todo).await.unwrap();
    let (status, json) = clear_todo_list(axum::Extension(Arc::new(dao))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json.0.text, "All todos have been deleted");
}

#[tokio::test]
async fn test_delete_todo() {
    let dao = TodoListDao::new().await.unwrap();
    dao.initialize().await;
    let todo = backend::Todo {
        id: 1,
        title: "Test Delete".to_string(),
        priority: 1,
        completed: false,
    };
    dao.save_todo(&todo).await.unwrap();
    let (status, json) = delete_todo(axum::Extension(Arc::new(dao)), axum::Json(IdPayload { id: 1 })).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json.0.text, "Todo with id 1 deleted successfully");
}
