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

use crate::common::api::{encode, error_response, parse_json, return_error};
use crate::common::db::connect_db;
use crate::{AppState, RegistrationFlow};

use entity::accounts::ActiveModel;

use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, Set};
use serde::Deserialize;
use serde_json::{json, Value};
use webauthn_rs::prelude::*;

#[derive(Deserialize)]
struct StartPayload {
    username: String,
}

pub async fn start(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let payload = match parse_json::<StartPayload>(payload).await {
        Ok(payload) => payload,
        Err(error) => return error,
    };

    let uuid = Uuid::now_v7();
    let flow_id = Uuid::now_v7();
    let (ccr, registration_state) =
        match state
            .rp
            .start_passkey_registration(uuid, &payload.username, &payload.username, None)
        {
            Ok((ccr, registration_state)) => (ccr, registration_state),
            Err(error) => return return_error(StatusCode::INTERNAL_SERVER_ERROR, error),
        };

    state.auth_flows.register.insert(
        flow_id,
        RegistrationFlow {
            registration_state,
            username: payload.username,
            uuid,
        },
    );

    (
        StatusCode::OK,
        Json(json!({
            "flow_id": flow_id,
            "ccr": ccr
        })),
    )
}

#[derive(Deserialize)]
struct FinishPayload {
    flow_id: Uuid,
    registration: RegisterPublicKeyCredential,
}

pub async fn finish(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let payload = match parse_json::<FinishPayload>(payload).await {
        Ok(payload) => payload,
        Err(error) => return error,
    };

    let db = match connect_db().await {
        Ok(database_connection) => database_connection,
        Err(error) => return return_error(StatusCode::INTERNAL_SERVER_ERROR, error),
    };

    let flow = match state.auth_flows.register.get(&payload.flow_id) {
        Some(flow) => flow,
        None => {
            return error_response(
                StatusCode::BAD_REQUEST,
                String::from("authentication flow not found"),
            )
        }
    };

    let sk = match state
        .rp
        .finish_passkey_registration(&payload.registration, &flow.registration_state)
    {
        Ok(sk) => sk,
        Err(error) => return return_error(StatusCode::INTERNAL_SERVER_ERROR, error),
    };

    let b64 = match encode(&sk).await {
        Ok(passkey) => passkey,
        Err(error) => return error,
    };

    let account = ActiveModel {
        id: Set(flow.uuid),
        name: Set(String::from(&flow.username)),
        verifier: Set(b64),
    };

    match account.insert(&db).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "sk": sk
            })),
        ),
        Err(error) => return_error(StatusCode::INTERNAL_SERVER_ERROR, error),
    }
}
