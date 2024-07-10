mod common;
mod routes;

use argon2::password_hash::rand_core::OsRng;
use axum::{
    routing::{get, post},
    Router,
};
use opaque_ke::keypair::PrivateKey;
use opaque_ke::{Ristretto255, ServerSetup};
use common::db;
use migration::{Migrator, MigratorTrait};
use crate::common::opaque::Default;

use routes::auth::register::finalize::main as finalize;
use routes::auth::register::request::main as request;
use routes::get_emails::main as mails;
use routes::get_users::main as users;
use routes::receive::main as send;

#[derive(Clone)]
pub struct AppState {
    pub server_setup: ServerSetup<Default, PrivateKey<Ristretto255>>,
}

#[tokio::main]
async fn main() {
    db::create_db().await;

    let mut rng = OsRng;
    let server_setup = ServerSetup::<Default>::new(&mut rng);

    let state = AppState { server_setup };
    
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
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:25052")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
