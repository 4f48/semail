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

use crate::common::api::{
    b64_decode, deserialize, encode, error_response, parse_json, return_error,
};
use crate::common::db::check_if_exists;
use crate::common::opaque::Default;
use crate::common::state::AppState;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use opaque_ke::ServerRegistration;
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
    let payload = match parse_json::<Payload>(payload).await {
        Ok(payload) => payload,
        Err(error) => return error,
    };

    match check_if_exists(&payload.username).await {
        Ok(true) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                String::from("user with this name already exists"),
            )
        }
        Ok(false) => (),
        Err(error) => return return_error(StatusCode::INTERNAL_SERVER_ERROR, error),
    };

    let bytes = match b64_decode(&payload.request).await {
        Ok(bytes) => bytes,
        Err(error) => return error,
    };
    let message = match deserialize(&bytes).await {
        Ok(message) => message,
        Err(error) => return error,
    };

    let result = match ServerRegistration::<Default>::start(
        &state.server_setup,
        message,
        payload.username.as_bytes(),
    ) {
        Ok(response) => response,
        Err(error) => return return_error(StatusCode::INTERNAL_SERVER_ERROR, error),
    };

    let response = match encode(&result.message).await {
        Ok(response) => response,
        Err(error) => return error,
    };

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
