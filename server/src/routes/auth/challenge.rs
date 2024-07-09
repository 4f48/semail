use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub async fn main(Json(_payload): Json<Value>) -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({}))
    )
}