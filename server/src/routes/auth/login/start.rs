/*
 * This file is part of SE-Mail.
 *
 * Copyright © 2024 Oliver Pirger <0x4f48@proton.me>
 *
 * SE-Mail is free software: you can redistribute it and/or modify it under the terms of the
 * GNU General Public License, version 3, as published by the Free Software Foundation.
 *
 * SE-Mail is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 *  without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 *  See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with SE-Mail.
 * If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-only
 */

use crate::common::db;
use crate::common::opaque::Default as DefaultSuite;
use crate::common::state::AppState;

use entity::accounts;
use entity::prelude::Accounts;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use opaque_ke::{CredentialRequest, ServerLogin, ServerLoginStartParameters, ServerRegistration};
use rand::rngs::OsRng;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

#[derive(Deserialize)]
struct Payload {
    username: String,
    request: String,
}

pub async fn main(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let payload: Payload = match serde_json::from_value(payload) {
        Ok(payload) => payload,
        Err(error) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };

    let db = match db::connect_db().await {
        Ok(database_connection) => database_connection,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };

    let results = match Accounts::find()
        .filter(accounts::Column::Name.eq(&payload.username))
        .all(&db)
        .await
    {
        Ok(results) => results,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };

    let verifier_b64 = match results.first() {
        Some(result) => result,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "this user doesn't exist"
                })),
            )
        }
    };

    let decoded = match BASE64_STANDARD.decode(&verifier_b64.verifier) {
        Ok(decoded) => decoded,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };
    let verifier: ServerRegistration<DefaultSuite> = match bincode::deserialize(&decoded) {
        Ok(verifier) => verifier,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };

    let decoded_req = match BASE64_STANDARD.decode(payload.request) {
        Ok(decoded) => decoded,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };
    let request: CredentialRequest<DefaultSuite> = match bincode::deserialize(&decoded_req) {
        Ok(request) => request,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };

    let mut server_rng = OsRng;
    let start_result = match ServerLogin::start(
        &mut server_rng,
        &state.server_setup,
        Some(verifier),
        request,
        payload.username.as_bytes(),
        ServerLoginStartParameters::default(),
    ) {
        Ok(start_result) => start_result,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };

    let serialized = match bincode::serialize(&start_result.state) {
        Ok(serialized) => serialized,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };
    let encoded = BASE64_STANDARD.encode(serialized);

    let flow_id = Uuid::now_v7();
    state.flows.login.insert(flow_id, String::from(&encoded));

    let serialized = match bincode::serialize(&start_result.message) {
        Ok(serialized) => serialized,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };
    let encoded = BASE64_STANDARD.encode(serialized);

    (
        StatusCode::OK,
        Json(json!({
            "flow_id": flow_id,
            "response": encoded
        })),
    )
}
