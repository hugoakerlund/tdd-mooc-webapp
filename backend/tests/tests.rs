use backend::{CreateTodo, 
              IdPayload, 
              create_todo, 
              toggle_todo_completion, 
              delete_todo,
              increase_todo_priority,
              decrease_todo_priority,
              clear_todo_list,
              root};
use backend::todo_list_dao::TodoListDao;
use axum::http::StatusCode;
use sqlx::Row;
use std::sync::Arc;

#[tokio::test]
async fn test_root_returns_welcome_message() {
    let res = root().await;
    assert_eq!(res.0.text, "Welcome to Rust API");
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

#[tokio::test]
async fn test_create_dao() {
    let dao: TodoListDao = TodoListDao::new().await.unwrap();
    assert_eq!(dao.is_open(), true, "Expected database connection to be open");
}

#[tokio::test]
async fn test_dao_create_table() {
    let dao: TodoListDao = TodoListDao::new().await.unwrap();
    let result = dao.create_table().await.unwrap();
    assert_eq!(result, "Database table created successfully", "Expected table creation success message");
}

#[tokio::test]
async fn test_dao_query_todos_when_empty() {
    let dao: TodoListDao = TodoListDao::new().await.unwrap();
    dao.initialize().await;
    let todos = dao.query_todos().await.unwrap();
    println!("Queried todos: {:?}", todos);
    assert_eq!(todos.len(), 0, "Expected no todos in the database");
}

#[tokio::test]
async fn test_dao_save_todo() {
    let todo = backend::Todo {
        id: 0,
        title: "Test Save".to_string(),
        priority: 1,
        completed: false,
    };
    let dao: TodoListDao = TodoListDao::new().await.unwrap();
    dao.initialize().await;
    let result: u32 = dao.save_todo(&todo).await.unwrap();
    assert_eq!(result, 1, "Expected one row to be affected when saving a todo");
}

#[tokio::test]
async fn test_dao_truncate_tables() {
    let todo = backend::Todo {
        id: 1,
        title: "Test truncate".to_string(),
        priority: 1,
        completed: false,
    };
    let dao: TodoListDao = TodoListDao::new().await.unwrap();
    dao.initialize().await;
    let before_save = dao.query_todos().await.unwrap();
    println!("Todos before save: {:?}", before_save);
    assert_eq!(before_save.len(), 0, "Expected no todos in the database before saving");

    dao.save_todo(&todo).await.unwrap();
    let after_save = dao.query_todos().await.unwrap();
    println!("Todos after save: {:?}", after_save);
    assert_eq!(after_save.len(), 1, "Expected todos in the database after saving");

    dao.truncate_tables().await.unwrap();
    let after_truncate = dao.query_todos().await.unwrap();
    assert_eq!(after_truncate.len(), 0, "Expected no todos in the database after truncating");
}

#[tokio::test]
async fn test_dao_delete_todo() {
    let todo = backend::Todo {
        id: 1,
        title: "Test Delete".to_string(),
        priority: 1,
        completed: false,
    };
    let dao: TodoListDao = TodoListDao::new().await.unwrap();
    dao.initialize().await;
    dao.save_todo(&todo).await.unwrap();
    let todos_after_save = dao.query_todos().await.unwrap();
    println!("Todos after save: {:?}", todos_after_save);
    assert_eq!(!dao.is_empty(), true, "Expected todos in the database after saving");
    assert_eq!(todos_after_save.len(), 1, "Expected one todo in the database after saving");

    let todo_id = todos_after_save[0].get::<i32, _>("id") as u64;
    dao.delete_todo(todo_id).await.unwrap();
    let todos_after_delete = dao.query_todos().await.unwrap();
    assert_eq!(todos_after_delete.len(), 0, "Expected no todos in the database after deletion");
}

#[tokio::test]
async fn test_dao_change_todo_to_completed() {
    let todo = backend::Todo {
        id: 1,
        title: "Test Complete".to_string(),
        priority: 1,
        completed: false,
    };
    let dao: TodoListDao = TodoListDao::new().await.unwrap();
    dao.initialize().await;
    dao.save_todo(&todo).await.unwrap();
    let todos_after_save = dao.query_todos().await.unwrap();
    println!("Todos after save: {:?}", todos_after_save);

    assert_eq!(todos_after_save.len(), 1, "Expected one todo in the database after saving");
    let todo_id = todos_after_save[0].get::<i32, _>("id") as u64;
    dao.toggle_todo_completion(todo_id).await.unwrap();
    let todos_after_update = dao.query_todos().await.unwrap();
    println!("Todos after update: {:?}", todos_after_update);
    let completed_status = todos_after_update[0].get::<bool, _>("completed");
    assert_eq!(completed_status, true, "Expected the todo to be marked as completed");
}

#[tokio::test]
async fn test_dao_increase_todo_priority() {
    let todo = backend::Todo {
        id: 1,
        title: "Low Priority".to_string(),
        priority: 1,
        completed: false,
    };
    let dao = TodoListDao::new().await.unwrap();
    dao.initialize().await;
    dao.save_todo(&todo).await.unwrap();
    let todos_before_increase = dao.query_todos().await.unwrap();
    println!("Todos before priority increase: {:?}", todos_before_increase);

    dao.increase_todo_priority(todo.id as u64).await.unwrap();
    let todos_after_increase = dao.query_todos().await.unwrap();
    println!("Todos after priority increase: {:?}", todos_after_increase);
    let increased_priority = todos_after_increase[0].get::<i32, _>("priority");
    assert_eq!(increased_priority, 2, "Expected the todo priority to be increased by 1");
}

#[tokio::test]
async fn test_dao_decrease_todo_priority() {
    let todo = backend::Todo {
        id: 1,
        title: "High Priority".to_string(),
        priority: 5,
        completed: false,
    };
    let dao = TodoListDao::new().await.unwrap();
    dao.initialize().await;
    dao.save_todo(&todo).await.unwrap();
    let todos_before_decrease = dao.query_todos().await.unwrap();
    println!("Todos before priority decrease: {:?}", todos_before_decrease);

    dao.decrease_todo_priority(todo.id as u64).await.unwrap();
    let todos_after_decrease = dao.query_todos().await.unwrap();
    println!("Todos after priority decrease: {:?}", todos_after_decrease);
    let decreased_priority = todos_after_decrease[0].get::<i32, _>("priority");
    assert_eq!(decreased_priority, 4, "Expected the todo priority to be decreased by 1");
}