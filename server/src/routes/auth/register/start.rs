use crate::common::db;
use crate::common::opaque::Default;
use crate::AppState;

use entity::accounts;
use entity::prelude::Accounts;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use base64::prelude::*;
use opaque_ke::{RegistrationRequest, ServerRegistration, ServerRegistrationStartResult};
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

    let decoded = BASE64_STANDARD.decode(payload.request).unwrap();
    let deserialized: RegistrationRequest<Default> = bincode::deserialize(&decoded).unwrap();

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
