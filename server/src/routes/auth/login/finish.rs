use crate::common::opaque::Default;
use crate::AppState;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use opaque_ke::{CredentialFinalization, ServerLogin};
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

    let uuid = match Uuid::from_str(&payload.flow_id) {
        Ok(uuid) => uuid,
        Err(error) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };
    let flow = match state.flows.login.get(&uuid) {
        Some(flow) => flow,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "this login flow doesn't exist"
                })),
            )
        }
    };
    state.flows.login.remove(&uuid);
    let decoded = match BASE64_STANDARD.decode(flow.value()) {
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
    let start_state: ServerLogin<Default> = match bincode::deserialize(&decoded) {
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

    let decoded = match BASE64_STANDARD.decode(payload.result) {
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
    let result: CredentialFinalization<Default> = match bincode::deserialize(&decoded) {
        Ok(result) => result,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };

    match start_state.finish(result) {
        Ok(_finish_result) => (
            StatusCode::OK,
            Json(json!({
                "success": "you logged in"
            })),
        ),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": format!("{}", error)
            })),
        ),
    }
}
