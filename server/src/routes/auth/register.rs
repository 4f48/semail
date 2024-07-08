use axum::http::StatusCode;
use axum::{Form, Json};
use axum::response::Redirect;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
pub struct FormData {
    address: String,
    password: String,
}

pub async fn main(Form(form_data): Form<FormData>) -> Redirect {
    let regex = Regex::new(r"^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !regex.is_match(&form_data.address) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "malformed se-mail address"
            })),
        );
    };
    
    Redirect::to("http://localhost:4321/")
}