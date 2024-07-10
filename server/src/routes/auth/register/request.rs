use crate::common::opaque::Default;
use crate::AppState;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use base64::{engine::general_purpose, Engine as _};
use opaque_ke::{RegistrationRequest, ServerRegistration};
use serde::Deserialize;
use serde_json::{json, Value};

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
                    "error": format!("{}", error.to_string())
                })),
            )
        }
    };

    let start = ServerRegistration::<Default>::start(
        &state.server_setup,
        match general_purpose::STANDARD.decode(payload.request) {
            Ok(request) => {
                let message: RegistrationRequest<Default> = match bincode::deserialize(&*request) {
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
    );

    todo!();
}
