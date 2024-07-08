use axum::http::StatusCode;
use axum::{Form, Json};
use axum::response::Redirect;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct FormData {
    address: String,
    password: String,
}

pub async fn main(Form(form_data): Form<FormData>) -> (StatusCode, Json<Value>) {
    let regex = Regex::new(r"^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !regex.is_match(&form_data.address) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "malformed se-mail address"
            })),
        );
    };
    
    // add more checks and database insert

    (
        StatusCode::OK,
        Json(json!({
            "success": "added account to registry"
        }))
    )
}