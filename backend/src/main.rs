use dotenvy::dotenv;
use std::net::SocketAddr;

use backend::{build_app};
use backend::todo_list_dao::TodoListDao;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let _database = TodoListDao::new().await.unwrap();

    let app = build_app();

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