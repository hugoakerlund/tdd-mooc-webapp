use dotenvy::dotenv;
use std::net::SocketAddr;

use backend::{build_app};
use backend::todo_list_dao::TodoListDao;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();
    // print all environment variables for debugging
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }

    // Initialize database pool (fail early with helpful message)
    let _database = TodoListDao::new().await.unwrap();

    let app = build_app();

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    let msg = format!("Server listening on http://{}", addr);
    println!("{}", msg);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}