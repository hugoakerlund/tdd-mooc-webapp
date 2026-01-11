use sqlx::{postgres::PgPoolOptions, Row};
use dotenvy::dotenv;
use crate::Todo;

pub struct TodoListDao {
    database: sqlx::Pool<sqlx::Postgres>,
}

impl TodoListDao {
    pub async fn new() -> Result<Self, sqlx::Error> {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(Self { database: pool })
    }

    
    pub fn is_open(&self) -> bool {
        !self.database.is_closed()
    }

    pub fn is_empty(&self) -> bool {
        self.database.size() == 0
    }

    pub async fn initialize(&self) {
        self.drop_todos_table().await.ok().unwrap(); 
        self.drop_archived_table().await.ok().unwrap();
        self.create_todos_table().await.ok().unwrap();
        self.create_archived_table().await.ok().unwrap();
    }

    pub async fn create_todos_table(&self) -> Result<&'static str, sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS todos (
                id SERIAL PRIMARY KEY,
                title TEXT NOT NULL,
                priority INT NOT NULL,
                completed BOOLEAN DEFAULT FALSE,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )"
        )
        .execute(&self.database)
        .await?;
        Ok("Database table created successfully")
    }

    pub async fn create_archived_table(&self) -> Result<&'static str, sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS archived (
                id SERIAL PRIMARY KEY,
                title TEXT NOT NULL,
                priority INT NOT NULL,
                completed BOOLEAN DEFAULT FALSE,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                archived_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )"
        )
        .execute(&self.database)
        .await?;
        Ok("Archived table created successfully")
    }

     pub async fn drop_todos_table(&self) -> Result<&'static str, sqlx::Error> {
        sqlx::query("DROP TABLE IF EXISTS todos")
            .execute(&self.database)
            .await?;
        Ok("All tables dropped successfully")
    }

    pub async fn drop_archived_table(&self) -> Result<&'static str, sqlx::Error> {
        sqlx::query("DROP TABLE IF EXISTS archived")
            .execute(&self.database)
            .await?;
        Ok("Archived table dropped successfully")
    }

    pub async fn truncate_todos_table(&self) -> Result<&'static str, sqlx::Error> {
        sqlx::query("TRUNCATE TABLE todos")
            .execute(&self.database)
            .await?;
        Ok("All tables truncated successfully")
    }
   
    pub async fn query_todos(&self) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
        println!("Querying todos from the database...");
        let todos: Vec<sqlx::postgres::PgRow> = sqlx::query("
            SELECT id, title, priority, completed
            FROM todos
            ORDER BY priority DESC, created_at ASC")
            .fetch_all(&self.database)
            .await?;
        Ok(todos)
    }

    pub async fn query_archived_todos(&self) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
        println!("Querying archived todos from the database...");
        let archived_todos: Vec<sqlx::postgres::PgRow> = sqlx::query("
            SELECT id, title, priority, completed, archived_at
            FROM archived
            ORDER BY archived_at DESC")
            .fetch_all(&self.database)
            .await?;
        Ok(archived_todos)
    }

    pub async fn save_todo(&self, todo: &Todo) -> Result<u32, sqlx::Error> {
        println!("Saving todo to the database...");
        let row = sqlx::query(
            "INSERT INTO todos (title, priority, completed) VALUES ($1, $2, $3) RETURNING id"
        )
        .bind(&todo.title)
        .bind(todo.priority as i32)
        .bind(todo.completed)
        .fetch_one(&self.database)
        .await?;

        let id: i32 = row.get("id");
        Ok(id as u32)
    }

    pub async fn archive_completed_todos(&self) -> Result<u64, sqlx::Error> {
        println!("Archiving completed todos...");
        let result = sqlx::query(
            "INSERT INTO archived (title, priority, completed)
             SELECT title, priority, completed FROM todos WHERE completed = TRUE"
        )
        .execute(&self.database)
        .await?;

        sqlx::query("DELETE FROM todos WHERE completed = TRUE")
            .execute(&self.database)
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn rename_todo(&self, todo_id: u64, new_title: String) -> Result<u32, sqlx::Error> {
        println!("Renaming todo with id {} in the database...", todo_id);
        sqlx::query("UPDATE todos SET title = $1 WHERE id = $2")
            .bind(new_title)
            .bind(todo_id as i32)
            .execute(&self.database)
            .await?;
        Ok(todo_id as u32)
    }

    pub async fn delete_todo(&self, todo_id: u64) -> Result<u64, sqlx::Error> {
        println!("Deleting todo with id {} from the database...", todo_id);
        let result = sqlx::query("DELETE FROM todos WHERE id = $1")
            .bind(todo_id as i32)
            .execute(&self.database)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn toggle_todo_completion(&self, todo_id: u64) -> Result<u32, sqlx::Error> {
        println!("Changing todo with id {} to completed in the database...", todo_id);
        sqlx::query("UPDATE todos SET completed = NOT completed WHERE id = $1")
            .bind(todo_id as i32)
            .execute(&self.database)
            .await?;
        Ok(todo_id as u32)
    }

    pub async fn increase_todo_priority(&self, todo_id: u64) -> Result<u32, sqlx::Error> {
        println!("Increasing priority of todo with id {} in the database...", todo_id);
        sqlx::query("UPDATE todos SET priority = priority + 1 WHERE id = $1")
            .bind(todo_id as i32)
            .execute(&self.database)
            .await?;
        Ok(todo_id as u32)
    }

    pub async fn decrease_todo_priority(&self, todo_id: u64) -> Result<u32, sqlx::Error> {
        println!("Decreasing priority of todo with id {} in the database...", todo_id);
        sqlx::query("UPDATE todos SET priority = priority - 1 WHERE id = $1")
            .bind(todo_id as i32)
            .execute(&self.database)
            .await?;
        Ok(todo_id as u32)
    }
}