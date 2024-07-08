use axum::http::StatusCode;
use axum::{Form, Json};
use axum::response::Redirect;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
pub struct FormData {
    address: String,
    password: String,
}

pub async fn main(Form(form_data): Form<FormData>) -> Redirect {
    Redirect::to("http://localhost:4321/")
}