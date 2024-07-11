mod common;
mod routes;

use common::db;
use common::opaque::Default;
use routes::auth::register::finish::main as finish;
use routes::auth::register::start::main as start;
use routes::get_emails::main as mails;
use routes::get_users::main as users;
use routes::receive::main as send;

use migration::{Migrator, MigratorTrait};

use crate::common::opaque::server_setup;
use argon2::password_hash::rand_core::OsRng;
use axum::{
    routing::{get, post},
    Router,
};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use dashmap::DashMap;
use opaque_ke::keypair::PrivateKey;
use opaque_ke::{Ristretto255, ServerSetup};
use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub server_setup: ServerSetup<Default, PrivateKey<Ristretto255>>,
    pub flows: Flows,
}

#[derive(Clone)]
pub struct Flows {
    register: DashMap<Uuid, String>,
    _login: DashMap<Uuid, String>, // prepare for login
}

#[tokio::main]
async fn main() {
    db::create_db().await;

    let server_setup = server_setup().await;

    let state = AppState {
        server_setup,
        flows: Flows {
            register: DashMap::new(),
            _login: DashMap::new(),
        },
    };

    Migrator::up(&db::connect_db().await.unwrap(), None)
        .await
        .unwrap();

    let app = Router::new()
        .route("/send", post(send))
        .route("/auth/register/start", post(start))
        .route("/auth/register/finish", post(finish))
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
