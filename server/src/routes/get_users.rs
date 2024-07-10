use crate::common::db;
use axum::Json;
use serde_json::{json, Value};

pub async fn main() -> Json<Value> {
    match db::get_accounts().await {
        Ok(accounts) => Json(json!(accounts)),
        Err(error) => Json(json!({
            "Oh no!": "Something went wrong",
            "error": format!("{:?}", error),
        })),
    }
}
