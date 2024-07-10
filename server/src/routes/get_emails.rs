use crate::common::db;
use axum::Json;
use serde_json::{json, Value};
use uuid::uuid;

pub async fn main() -> Json<Value> {
    match db::get_emails(uuid!("019083d2-86c7-7d22-947d-b4c3937db73b")).await {
        Ok(mails) => Json(json!(mails)),
        Err(error) => Json(json!({
            "Oh no!": "Something went wrong",
            "error": format!("{:?}", error),
        })),
    }
}
