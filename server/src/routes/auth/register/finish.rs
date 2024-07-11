use crate::common::db;
use crate::common::opaque::Default;
use crate::AppState;

use entity::accounts::ActiveModel;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
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
    let payload: Payload = match serde_json::from_value(payload) {
        Ok(payload) => payload,
        Err(error) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": format!("0 {}", error)
                })),
            )
        }
    };

    let decoded = BASE64_STANDARD.decode(payload.result).unwrap();
    dbg!(&decoded);
    let deserialized = match bincode::deserialize(&decoded) {
        Ok(upload) => upload,
        Err(error) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": format!("2 {}", error)
                })),
            )
        }
    };

    /*let deserialized = match bincode::deserialize(match &BASE64_STANDARD.decode(payload.result) {
        Ok(result) => {
            dbg!(result);
            result
        },
        Err(error) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                        "error": format!("1 {}", error)
                    })),
            )
        }
    }) {
        Ok(upload) => upload,
        Err(error) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                        "error": format!("2 {}", error)
                    })),
            )
        }
    };*/

    let verifier = ServerRegistration::<Default>::finish(deserialized);

    let uuid = match Uuid::from_str(&payload.flow_id) {
        Ok(uuid) => uuid,
        Err(error) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": format!("3 {}", error)
                })),
            )
        }
    };

    let name = match state.flows.register.get(&uuid) {
        Some(username) => username,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "this registration flow doesn't exist"
                })),
            )
        }
    };

    let account = ActiveModel {
        id: Set(Uuid::now_v7()),
        name: Set(name.value().to_string()),
        verifier: Set(BASE64_STANDARD.encode(match bincode::serialize(&verifier) {
            Ok(verifier) => verifier,
            Err(error) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": format!("{}", error)
                    })),
                )
            }
        })),
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

    match account.insert(&db).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "success": "user registration successfully completed"
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
