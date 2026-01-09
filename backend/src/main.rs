use dotenvy::dotenv;
use std::net::SocketAddr;

use backend::{build_app, create_pool};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();
    // print all environment variables for debugging
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }

    // Initialize database pool (fail early with helpful message)
    let _db_pool = match create_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to create pool: {}. Check DATABASE_URL and credentials.", e);
            std::process::exit(1);
        }
    };

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