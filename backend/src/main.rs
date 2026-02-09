use dotenvy::dotenv;
use std::net::SocketAddr;

use backend::{build_app};
use backend::todo_list_dao::TodoListDao;
use std::sync::Arc;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let database = TodoListDao::new().await.unwrap();
    database.initialize().await;
    let db = Arc::new(database);

    let app = build_app(db.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    let msg = format!("Server listening on http://{}", addr);
    println!("{}", msg);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}