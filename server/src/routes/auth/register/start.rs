use crate::common::opaque::Default;
use crate::AppState;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use base64::prelude::*;
use opaque_ke::{RegistrationRequest, ServerRegistration};
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;
use entity::accounts;
use entity::prelude::Accounts;
use crate::common::db;

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

    match Accounts::find()
        .filter(accounts::Column::Name.eq(&payload.username))
        .all(&db)
        .await
    {
        Ok(results) => match results.first() {
            None => {
                match ServerRegistration::<Default>::start(
                    &state.server_setup,
                    match BASE64_STANDARD.decode(payload.request) {
                        Ok(request) => {
                            let message: RegistrationRequest<Default> = match bincode::deserialize(&request) {
                                Ok(message) => message,
                                Err(error) => {
                                    return (
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        Json(json!({
                                "error": format!("{}", error)
                            })),
                                    )
                                }
                            };
                            message
                        }
                        Err(error) => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({
                        "error": format!("{}", error)
                    })),
                            )
                        }
                    },
                    payload.username.as_bytes(),
                ) {
                    Ok(server_start_result) => {
                        let response =
                            BASE64_STANDARD.encode(match bincode::serialize(&server_start_result.message) {
                                Ok(response) => response,
                                Err(error) => {
                                    return (
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        Json(json!({
                                "error": format!("{}", error)
                            })),
                                    )
                                }
                            });

                        let flow_id = Uuid::now_v7();
                        let mut register_flows = match state.register_flows.lock() {
                            Ok(register_flow) => register_flow,
                            Err(error) => return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({
                        "error": format!("{}", error)
                    }))
                            )
                        };
                        register_flows.insert(flow_id, payload.username).to_owned();

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
            },
            Some(_) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "a user with this name already exists"
                }))
            )
        },
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": format!("{}", error)
            }))
        )
    }
}
