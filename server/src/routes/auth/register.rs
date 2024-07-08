use axum::extract::Multipart;
use axum::http::StatusCode;
use axum::response::Redirect;
use axum::{Form, Json};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    username: String,
    password: String,
}

pub async fn main(Json(payload): Json<Value>) -> (StatusCode, Json<Value>) {
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

    // add more checks and database insert

    (
        StatusCode::OK,
        Json(json!({
            "success": "added account to registry"
        })),
    )
}
