use crate::common::opaque::Default;
use axum::http::StatusCode;
use axum::Json;
use opaque_ke::ServerSetup;
use serde::Deserialize;
use serde_json::{json, Value};
use axum::extract::State;

#[derive(Deserialize)]
struct Payload {
    _username: String,
    _request: String,
}

pub async fn main(Json(payload): Json<Value>) -> (StatusCode, Json<Value>) {
    let _payload: Payload = match serde_json::from_value(payload) {
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

    todo!();
}
