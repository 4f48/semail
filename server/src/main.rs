mod common;
mod routes;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use common::db;
use common::opaque::Default;
use routes::auth::register::finish::main as finish;
use routes::auth::register::start::main as start;
use routes::get_emails::main as mails;
use routes::get_users::main as users;
use routes::receive::main as send;

use migration::{Migrator, MigratorTrait};

use argon2::password_hash::rand_core::OsRng;
use axum::{
    routing::{get, post},
    Router,
};
use opaque_ke::keypair::PrivateKey;
use opaque_ke::{Ristretto255, ServerSetup};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub server_setup: ServerSetup<Default, PrivateKey<Ristretto255>>,
    pub register_flows: Arc<Mutex<HashMap<Uuid, String>>>,
    pub login_flows: Arc<Mutex<HashMap<Uuid, String>>> // prepare for login flows session storage
}

#[tokio::main]
async fn main() {
    db::create_db().await;

    let mut rng = OsRng;
    let server_setup = ServerSetup::<Default>::new(&mut rng);

    let state = AppState { 
        server_setup,
        register_flows: Arc::new(Mutex::new(HashMap::new().to_owned())),
        login_flows: Arc::new(Mutex::new(HashMap::new().to_owned()))
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
