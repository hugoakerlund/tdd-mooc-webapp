use sqlx::{Acquire, database, postgres::PgPoolOptions};
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

    pub async fn create_table(&self) -> Result<&'static str, sqlx::Error> {
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

    pub async fn query_todos(&self) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
        let todos = sqlx::query("
            SELECT id, title, priority
            FROM todos
            WHERE title ILIKE $1
            ORDER BY priority DESC, created_at ASC")
            .fetch_all(&self.database)
            .await?;
        Ok(todos)
    }

    pub async fn save_todo(&self, todo: &Todo) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO todos (title, priority) VALUES ($1, $2)"
        )
        .bind(&todo.name)
        .bind(todo.priority as i32)
        .execute(&self.database)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn trucate_tables(&self) -> Result<&'static str, sqlx::Error> {
        sqlx::query("TRUNCATE TABLE todos")
            .execute(&self.database)
            .await?;
        Ok("All tables truncated successfully")
    }

    pub async fn delete_todo(&self, todo_id: u64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM todos WHERE id = $1")
            .bind(todo_id as i32)
            .execute(&self.database)
            .await?;
        Ok(result.rows_affected())
    }
}