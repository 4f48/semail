use crate::db;
use axum::{Json};
use entity::mails::ActiveModel;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbErr};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use uuid::{uuid, Uuid};

#[derive(Serialize, Deserialize)]
struct Payload {
    from: String,
    to: String,
    subject: String,
    body: String,
}

pub async fn main(Json(payload): Json<Value>) -> Json<Value> {
    let payload: Payload = match serde_json::from_value(payload) {
        Ok(payload) => payload,
        Err(error) => {
            return Json(json!({
            "error": "mail wasn't forwarded to recipient",
            "reason": "the 'from' and or 'to' value(s) are missing from json body",
            "detailed_error": format!("{}", error),
        }))
        }
    };
    
    match insert(
        payload.from,
        payload.to,
        Option::from(payload.subject),
        Option::from(payload.body),
    )
    .await
    {
        Ok(_) => Json(json!({
            "success": format!("email forwarded to recipient"),
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
        owner: Set(uuid!("019083d2-86c7-7d22-947d-b4c3937db73b")), // hard coded for now, query uuid from username and instance IP/domain in the future
        from: Set(from),
        to: Set(to),
        subject: Set(subject),
        body: Set(body),
        ..Default::default()
    };

    mail.insert(&db).await?;

    Ok(())
}
