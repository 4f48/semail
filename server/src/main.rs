mod common;
mod routes;

use axum::{
    routing::{get, post},
    Router,
};

use common::db;
use migration::{Migrator, MigratorTrait};

use routes::auth::register::finalize::main as finalize;
use routes::auth::register::request::main as request;
use routes::get_emails::main as mails;
use routes::get_users::main as users;
use routes::receive::main as send;

use common::state::main as create_state;

#[tokio::main]
async fn main() {
    db::create_db().await;

    Migrator::up(&db::connect_db().await.unwrap(), None)
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/send", post(send))
        .route("/auth/register/request", post(request))
        .route("/auth/register/finalize", post(finalize))
        .route(
            "/test",
            get(|| async { db::create_test_user().await.unwrap() }),
        )
        .route("/get", get(users))
        .route("/mails", get(mails))
        .with_state(create_state());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:25052")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
