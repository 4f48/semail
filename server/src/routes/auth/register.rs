use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub async fn main() -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "success": "user added to registry"
        }))
    )
}