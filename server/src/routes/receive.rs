use crate::db;
use axum::http::StatusCode;
use axum::Json;
use entity::accounts;
use entity::mails::ActiveModel;
use entity::prelude::Accounts;
use regex::Regex;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Payload {
    from: String,
    to: String,
    subject: String,
    body: String,
}

pub async fn main(Json(payload): Json<Value>) -> (StatusCode, Json<Value>) {
    let payload: Payload = match serde_json::from_value(payload) {
        Ok(payload) => payload,
        Err(error) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": format!("{}", error.to_string())
                })),
            )
        }
    };

    let regex = Regex::new(r"^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !regex.is_match(&payload.to) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "malformed se-mail address"
            })),
        );
    };

    struct Recipient {
        account: String,
        instance: String,
    }

    let recipient = payload.to.split('@').collect::<Vec<&str>>();
    let recipient = Recipient {
        account: recipient[0].parse().unwrap(),
        instance: recipient[1].parse().unwrap(),
    };

    // example.dev hardcoded for testing, acquire INSTANCE_URL environment variable later
    if recipient.instance != "example.dev" {
        return (
            StatusCode::MISDIRECTED_REQUEST,
            Json(json!({
                "error": format!("wrong instance, this is example.dev, not {}", recipient.instance)
            })),
        );
    }

    let db = match db::connect_db().await {
        Ok(database_connection) => database_connection,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };

    let uuid = match Accounts::find()
        .filter(accounts::Column::Name.eq(recipient.account))
        .all(&db)
        .await
    {
        Ok(results) => match results.first() {
            Some(account) => account.id,
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "this account doesn't exist here"
                    })),
                )
            }
        },
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };

    let mail = ActiveModel {
        id: Set(Uuid::now_v7()),
        owner: Set(uuid),
        from: Set(payload.from),
        to: Set(payload.to),
        subject: Set(Option::from(payload.subject)),
        body: Set(Option::from(payload.body)),
        ..Default::default()
    };

    match mail.insert(&db).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "success": "mail forwarded to recipient"
            })),
        ),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": format!("{}", error)
            })),
        ),
    }
}
