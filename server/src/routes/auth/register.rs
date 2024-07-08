use std::fmt::Debug;
use axum::http::StatusCode;
use axum::{Form, Json};
use axum::extract::Multipart;
use axum::response::Redirect;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    address: String,
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

    dbg!(payload);
    
    /*
    let regex = Regex::new(r"^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !regex.is_match(mu.address) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "malformed se-mail address"
            })),
        );
    };
    */
    
    // add more checks and database insert

    (
        StatusCode::OK,
        Json(json!({
            "success": "added account to registry"
        }))
    )
}