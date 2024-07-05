mod routes;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::migrate::MigrateDatabase;
use sqlx::Sqlite;

use routes::send::main as send;

#[tokio::main]
async fn main() {
    create_db().await;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/send", post(send));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:25052")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

const DB_URL: &str = "sqlite://sqlite.db";
async fn create_db() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Successfully created database"),
            Err(error) => panic!("Something went wrong while creating database: {}", error),
        }
    }
}
