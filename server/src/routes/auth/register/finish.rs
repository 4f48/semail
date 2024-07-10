use std::str::FromStr;
use crate::common::opaque::Default;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use opaque_ke::ServerRegistration;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::{Uuid, uuid};
use entity::accounts::ActiveModel;
use rand::rngs::OsRng;
use entity::accounts;
use entity::prelude::Accounts;
use crate::common::db;

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

    let register_flows = match state.register_flows.lock() {
        Ok(register_flows) => register_flows,
        Err(error) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": format!("{}", error)
            }))
        )
    };
    let name = match register_flows.get(match &Uuid::from_str(&payload.flow_id) {
        Ok(uuid) => uuid,
        Err(error) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": format!("{}", error)
            }))
        )
    }) {
        Some(username) => username,
        None => return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "this registration flow doesn't exist"
            }))
        )
    };

    let account = ActiveModel {
        id: Set(Uuid::now_v7()),
        name: Set(name.to_string()),
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
            }))
        ),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": format!("{}", error)
            }))
        )
    }
}
