mod db;
mod routes;

use axum::{
    routing::{get, post},
    Router,
};
use migration::{Migrator, MigratorTrait};

use routes::auth::register::main as register;
use routes::get_emails::main as mails;
use routes::get_users::main as users;
use routes::receive::main as send;

#[tokio::main]
async fn main() {
    db::create_db().await;

    Migrator::up(&db::connect_db().await.unwrap(), None)
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/send", post(send))
        .route("/auth/register", post(register))
        .route(
            "/test",
            get(|| async { db::create_test_user().await.unwrap() }),
        )
        .route("/get", get(users))
        .route("/mails", get(mails));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:25052")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
