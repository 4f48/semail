use crate::common::opaque::Default;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use opaque_ke::ServerRegistration;
use sea_orm::Set;
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;
use entity::accounts::ActiveModel;
use rand::rngs::OsRng;

#[derive(Deserialize)]
struct Payload {
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

    let verifier = ServerRegistration::<Default>::finish(
        match bincode::deserialize(match &BASE64_STANDARD.decode(payload.result) {
            Ok(result) => result,
            Err(error) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": format!("{}", error)
                    })),
                )
            }
        }) {
            Ok(upload) => upload,
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

    let account = ActiveModel {
        id: Set(Uuid::now_v7()),
        name: Set(String::new()), // whyyyyyyyyyyyyyyy
        verifier: Set(BASE64_STANDARD.encode(match bincode::serialize(&verifier) {
            Ok(verifier) => verifier,
            Err(error) => return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", error)
                }))
            )
        })),
    };

    todo!();
}
