mod db;
mod routes;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::migrate::MigrateDatabase;

use routes::send::main as send;

#[tokio::main]
async fn main() {
    db::create_db().await;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/send", post(send));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:25052")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
