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
use crate::common::opaque::Default;
use crate::AppState;

use entity::accounts;
use entity::prelude::Accounts;

use crate::common::api::{b64_decode, deserialize, error_response, parse_json};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use base64::prelude::*;
use opaque_ke::{RegistrationRequest, ServerRegistration};
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
    let payload = match parse_json::<Payload>(payload) {
        Ok(payload) => payload,
        Err(error) => return error,
    };

    let db = match db::connect_db().await {
        Ok(connection) => connection,
        Err(error) => return error_response(StatusCode::INTERNAL_SERVER_ERROR, error),
    };

    let decoded = match b64_decode(payload.request) {
        Ok(bytes) => bytes,
        Err(error) => return error,
    };

    let deserialized: RegistrationRequest<Default> = match deserialize(&decoded) {
        Ok(request) => request,
        Err(error) => return error,
    };

    match Accounts::find()
        .filter(accounts::Column::Name.eq(&payload.username))
        .all(&db)
        .await
    {
        Ok(results) => match results.first() {
            None => {
                match ServerRegistration::<Default>::start(
                    &state.server_setup,
                    deserialized,
                    payload.username.as_bytes(),
                ) {
                    Ok(server_start_result) => {
                        let response = BASE64_STANDARD.encode(
                            match bincode::serialize(&server_start_result.message) {
                                Ok(response) => response,
                                Err(error) => {
                                    return (
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        Json(json!({
                                            "error": format!("{}", error)
                                        })),
                                    )
                                }
                            },
                        );

                        let flow_id = Uuid::now_v7();
                        state.flows.register.insert(flow_id, payload.username);

                        (
                            StatusCode::OK,
                            Json(json!({
                                "flow_id": flow_id,
                                "response": response
                            })),
                        )
                    }
                    Err(error) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({
                            "error": format!("{}", error)
                        })),
                    ),
                }
            }
            Some(_) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "a user with this name already exists"
                })),
            ),
        },
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": format!("{}", error)
            })),
        ),
    }
}
