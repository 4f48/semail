mod common;
mod routes;

use common::db;
use common::opaque::Default;
use routes::auth::login::finish::main as login_finish;
use routes::auth::login::start::main as login_start;
use routes::auth::register::finish::main as register_finish;
use routes::auth::register::start::main as register_start;
use routes::get_emails::main as mails;
use routes::get_users::main as users;
use routes::receive::main as send;

use migration::{Migrator, MigratorTrait};

use crate::common::opaque::server_setup;
use axum::{
    routing::{get, post},
    Router,
};
use dashmap::DashMap;
use opaque_ke::keypair::PrivateKey;
use opaque_ke::{Ristretto255, ServerSetup};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub server_setup: ServerSetup<Default, PrivateKey<Ristretto255>>,
    pub flows: Flows,
}

#[derive(Clone)]
pub struct Flows {
    register: DashMap<Uuid, String>,
    login: DashMap<Uuid, String>,
}

#[tokio::main]
async fn main() {
    db::create_db().await;

    let server_setup = server_setup().await;

    let state = AppState {
        server_setup,
        flows: Flows {
            register: DashMap::new(),
            login: DashMap::new(),
        },
    };

    Migrator::up(&db::connect_db().await.unwrap(), None)
        .await
        .unwrap();

    let app = Router::new()
        .route("/send", post(send))
        .route("/auth/register/start", post(register_start))
        .route("/auth/register/finish", post(register_finish))
        .route("/auth/login/start", post(login_start))
        .route("/auth/login/finish", post(login_finish))
        // --- TESTING ROUTES, TO BE REMOVED ---
        .route(
            "/test",
            get(|| async { db::create_test_user().await.unwrap() }),
        )
        .route("/get", get(users))
        .route("/mails", get(mails))
        // ^^^ TESTING ROUTES, TO BE REMOVED ^^^
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:25052")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
