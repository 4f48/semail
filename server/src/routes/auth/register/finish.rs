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

use crate::common::api::{
    b64_decode, deserialize, encode, error_response, parse_json, return_error,
};
use crate::common::db::connect_db;
use crate::common::opaque::Default;
use crate::common::state::AppState;

use entity::accounts::ActiveModel;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use opaque_ke::ServerRegistration;
use sea_orm::{ActiveModelTrait, Set};
use serde::Deserialize;
use serde_json::{json, Value};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Deserialize)]
struct Payload {
    flow_id: String,
    result: String,
}

pub async fn main(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let payload = match parse_json::<Payload>(payload).await {
        Ok(payload) => payload,
        Err(error) => return error,
    };

    let uuid = match Uuid::from_str(&payload.flow_id) {
        Ok(uuid) => uuid,
        Err(error) => return return_error(StatusCode::BAD_REQUEST, error),
    };
    let flow = match state.flows.register.get(&uuid) {
        Some(flow) => flow,
        None => return error_response(StatusCode::BAD_REQUEST, String::from("invalid flow_id")),
    };

    let bytes = match b64_decode(&payload.result).await {
        Ok(bytes) => bytes,
        Err(error) => return error,
    };
    let result = match deserialize(&bytes).await {
        Ok(message) => message,
        Err(error) => return error,
    };
    let verifier = ServerRegistration::<Default>::finish(result);

    let account = ActiveModel {
        id: Set(Uuid::now_v7()),
        name: Set(flow.value().to_string()),
        verifier: Set(match encode(&verifier).await {
            Ok(result) => result,
            Err(error) => return error,
        }),
    };

    let db = match connect_db().await {
        Ok(db) => db,
        Err(error) => return return_error(StatusCode::INTERNAL_SERVER_ERROR, error),
    };

    match account.insert(&db).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "success": "user registration successfully completed"
            })),
        ),
        Err(error) => return_error(StatusCode::INTERNAL_SERVER_ERROR, error),
    }
}
