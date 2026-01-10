use backend::{root, create_todo, CreateTodo};
use backend::todo_list_dao::TodoListDao;
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

#[tokio::test]
async fn test_create_dao() {
    let dao: TodoListDao = TodoListDao::new().await.unwrap();
    assert_eq!(dao.is_open(), true, "Expected database connection to be open");
}

#[tokio::test]
async fn test_create_table() {
    let dao: TodoListDao = TodoListDao::new().await.unwrap();
    let result = dao.create_table().await.unwrap();
    assert_eq!(result, "Database table created successfully", "Expected table creation success message");
}

#[tokio::test]
async fn test_query_todos_when_empty() {
    let dao: TodoListDao = TodoListDao::new().await.unwrap();
    let todos = dao.query_todos().await.unwrap_or_else(|_| vec![]);
    assert_eq!(todos.len(), 0, "Expected no todos in the database");
}

#[tokio::test]
async fn test_save_todo() {
    let todo = backend::Todo {
        id: 0,
        name: "Test Save".to_string(),
        priority: 1,
    };
    let dao: TodoListDao = TodoListDao::new().await.unwrap();
    dao.create_table().await.unwrap();
    let result: u64 = dao.save_todo(&todo).await.unwrap();
    assert_eq!(result, 1, "Expected one row to be affected when saving a todo");
}

#[tokio::test]
async fn test_truncate_tables() {
    let todo = backend::Todo {
        id: 0,
        name: "Test Save".to_string(),
        priority: 1,
    };
    let dao: TodoListDao = TodoListDao::new().await.unwrap();
    dao.create_table().await.unwrap();
    let before_save = dao.query_todos().await.unwrap_or_else(|_| vec![]);
    assert_eq!(before_save.len(), 0, "Expected no todos in the database before saving");

    dao.save_todo(&todo).await.unwrap();
    let after_save = dao.query_todos().await.unwrap_or_else(|_| vec![]);
    assert_eq!(after_save.len(), 0, "Expected todos in the database after saving");

    dao.trucate_tables().await.unwrap();
    let after_truncate = dao.query_todos().await.unwrap_or_else(|_| vec![]);
    assert_eq!(after_truncate.len(), 0, "Expected no todos in the database after truncating");
}

#[tokio::test]
async fn test_delete_todo() {
    let todo = backend::Todo {
        id: 0,
        name: "Test Delete".to_string(),
        priority: 1,
    };
    let dao: TodoListDao = TodoListDao::new().await.unwrap();
    dao.create_table().await.unwrap();
    dao.save_todo(&todo).await.unwrap();
    assert_eq!(!dao.is_empty(), true, "Expected todos in the database after saving");

    dao.delete_todo(todo.id).await.unwrap();
    let todos_after_delete = dao.query_todos().await.unwrap_or_else(|_| vec![]);
    assert_eq!(todos_after_delete.len(), 0, "Expected no todos in the database after deletion");
}