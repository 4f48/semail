use crate::db;
use crate::db::connect_db;
use axum::{Json};
use entity::mails::ActiveModel;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbErr};
use serde_json::json;
use serde_json::Value;
use uuid::{uuid, Uuid};

pub async fn main(Json(payload): Json<Value>) -> Json<Value> {
    if payload["from"].is_null() || payload["to"].is_null() {
        return Json(json!({
            "error": "mail wasn't forwarded to recipient",
            "reason": "the 'from' and or 'to' value(s) are missing from json body"
        }))
    }
    
    // for some reason double quotes get included in strings too: FIX LATER
    match insert(
        payload["from"].to_string(),
        payload["to"].to_string(),
        Option::from(payload["subject"].to_string()),
        Option::from(payload["body"].to_string()),
    )
    .await
    {
        Ok(_) => Json(json!({
            "success": format!("email forwarded to recipient: {}", payload["to"]),
        })),
        Err(error) => Json(json!({
            "error": format!("{:?}", error),
        })),
    }
}

async fn insert(
    from: String,
    to: String,
    subject: Option<String>,
    body: Option<String>,
) -> Result<(), DbErr> {
    let db = db::connect_db().await?;
    let mail = ActiveModel {
        id: Set(Uuid::now_v7()),
        owner: Set(uuid!("019083d2-86c7-7d22-947d-b4c3937db73b")), // generate random for now, query user from username and instance IP/domain in the future
        from: Set(from),
        to: Set(to),
        subject: Set(subject),
        body: Set(body),
        ..Default::default()
    };

    mail.insert(&db).await?;

    Ok(())
}
