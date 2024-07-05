use axum::{extract, response};
use serde_json::{json, Value};

pub async fn main(
    extract::Json(payload): extract::Json<Value>,
) -> response::Json<Value> {
    let anything = &payload["Anything"];
    
    response::Json(payload)
}
