use backend::todo_list_dao::TodoListDao;
use sqlx::Row;


#[tokio::test]
async fn test_create_dao() {
    let dao: TodoListDao = TodoListDao::new().await.unwrap();
    assert_eq!(dao.is_open(), true, "Expected database connection to be open");
}

#[tokio::test]
async fn test_create_todos_table() {
    let dao: TodoListDao = TodoListDao::new().await.unwrap();
    let result = dao.create_todos_table().await.unwrap();
    assert_eq!(result, "Database table created successfully", "Expected table creation success message");
}

#[tokio::test]
async fn test_create_archived_table() {
    let dao: TodoListDao = TodoListDao::new().await.unwrap();
    let result = dao.create_archived_table().await.unwrap();
    assert_eq!(result, "Archived table created successfully", "Expected archived table creation success message");
}

#[tokio::test]
async fn test_query_todos_when_empty() {
    let dao: TodoListDao = TodoListDao::new().await.unwrap();
    dao.initialize().await;
    let todos = dao.query_todos().await.unwrap();
    println!("Queried todos: {:?}", todos);
    assert_eq!(todos.len(), 0, "Expected no todos in the database");
}

#[tokio::test]
async fn test_query_archived_todos_when_empty() {
    let dao: TodoListDao = TodoListDao::new().await.unwrap();
    dao.initialize().await;
    let archived_todos = dao.query_archived_todos().await.unwrap();
    println!("Queried archived todos: {:?}", archived_todos);
    assert_eq!(archived_todos.len(), 0, "Expected no archived todos in the database");
}

#[tokio::test]
async fn test_save_todo() {
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
async fn test_archive_completed_todos() {
    let todo1 = backend::Todo {
        id: 1,
        title: "Test Archive".to_string(),
        priority: 1,
        completed: true,
    };

    let todo2 = backend::Todo {
        id: 2,
        title: "Test Archive".to_string(),
        priority: 1,
        completed: true,
    };

    let dao: TodoListDao = TodoListDao::new().await.unwrap();
    dao.initialize().await;
    dao.save_todo(&todo1).await.unwrap();
    dao.save_todo(&todo2).await.unwrap();
    let queried_todos = dao.query_todos().await.unwrap();
    println!("Todos before archiving: {:?}", queried_todos);
    assert_eq!(queried_todos.len(), 2, "Expected two todos in the database before archiving");
    dao.archive_completed_todos().await.unwrap();

    let archived_todos = dao.query_archived_todos().await.unwrap();
    let queried_todos = dao.query_todos().await.unwrap();
    println!("Archived todos: {:?}", archived_todos);
    assert_eq!(archived_todos.len(), 2, "Expected two todos to be archived");
    assert_eq!(queried_todos.len(), 0, "Expected no todos in the active todos table");
}

#[tokio::test]
async fn test_rename_todo() {
    let todo = backend::Todo {
        id: 1,
        title: "Old Title".to_string(),
        priority: 1,
        completed: false,
    };
    let dao: TodoListDao = TodoListDao::new().await.unwrap();
    dao.initialize().await;
    dao.save_todo(&todo).await.unwrap();
    let todos_before_rename = dao.query_todos().await.unwrap();
    println!("Todos before rename: {:?}", todos_before_rename);

    dao.rename_todo(todo.id as u64, "New Title".to_string()).await.unwrap();
    let todos_after_rename = dao.query_todos().await.unwrap();
    println!("Todos after rename: {:?}", todos_after_rename);
    let renamed_title = todos_after_rename[0].get::<String, _>("title");
    assert_eq!(renamed_title, "New Title", "Expected the todo title to be updated");
}

#[tokio::test]
async fn test_truncate_todos_table() {
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

    dao.truncate_todos_table().await.unwrap();
    let after_truncate = dao.query_todos().await.unwrap();
    assert_eq!(after_truncate.len(), 0, "Expected no todos in the database after truncating");
}

#[tokio::test]
async fn test_delete_todo() {
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
async fn test_change_todo_to_completed() {
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
async fn test_increase_todo_priority() {
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
async fn test_decrease_todo_priority() {
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