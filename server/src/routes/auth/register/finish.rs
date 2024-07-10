use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;

pub async fn main(Json(_payload): Json<Value>) -> (StatusCode, Json<Value>) {
    todo!();
}
