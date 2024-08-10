/*
 * This file is part of SE-Mail.
 *
 * Copyright © 2024 Oliver Pirger <0x4f48@proton.me>
 *
 * SE-Mail is free software: you can redistribute it and/or modify it under the terms of the
 * GNU General Public License, version 3, as published by the Free Software Foundation.
 *
 * SE-Mail is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with SE-Mail.
 * If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-only
 */

mod common;
mod routes;

use common::db;
use dashmap::DashMap;
use routes::auth::register::finish as register_finish;
use routes::auth::register::start as register_start;
use routes::get_emails::main as mails;
use routes::get_users::main as users;
use routes::receive::main as send;
use routes::whodis::main as whodis;

use migration::{Migrator, MigratorTrait};

use axum::{
    routing::{get, post},
    Router,
};
use webauthn_rs::prelude::*;

#[derive(Clone)]
struct AppState {
    rp: Webauthn,
    auth_flows: AuthFlows,
}

#[derive(Clone)]
struct AuthFlows {
    register: DashMap<Uuid, RegistrationFlow>,
}

#[derive(Clone)]
struct RegistrationFlow {
    registration_state: PasskeyRegistration,
    username: String,
    uuid: Uuid,
}

#[tokio::main]
async fn main() {
    db::create_db().await;

    let id = "semail.4f48.dev";
    let origin = Url::parse("https://semail.4f48.dev").unwrap();

    // not sure if this "mut" is necessary, but documentation suggests it is for some reason
    let mut builder = WebauthnBuilder::new(id, &origin).unwrap();
    let rp = builder.build().unwrap();

    let state = AppState {
        rp,
        auth_flows: AuthFlows {
            register: DashMap::new(),
        },
    };

    Migrator::up(&db::connect_db().await.unwrap(), None)
        .await
        .unwrap();

    let app = Router::new()
        .route("/send", post(send))
        .route("/whodis", get(whodis))
        .route("/auth/register/start", post(register_start))
        .route("/auth/register/finish", post(register_finish))
        // --- TESTING ROUTES, TO BE REMOVED ---
        .route(
            "/test",
            get(|| async { db::create_test_user().await.unwrap() }),
        )
        .route("/get", get(users))
        .route("/mails", get(mails))
        // ^^^ TESTING ROUTES, TO BE REMOVED ^^^
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:26654")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
