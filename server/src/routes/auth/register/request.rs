use crate::common::opaque::Default;
use crate::common::state::AppState;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use opaque_ke::{RegistrationRequest, ServerRegistration};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
struct Payload {
    username: String,
    request: String,
}

pub async fn main(Json(payload): Json<Value>, State(state): State<AppState>) -> (StatusCode, Json<Value>) {
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
        payload.request,
        payload.username.as_bytes()
    );

    todo!();
}
