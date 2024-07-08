use crate::db;
use axum::http::StatusCode;
use axum::{Form, Json};
use entity::accounts;
use entity::accounts::ActiveModel;
use entity::prelude::Accounts;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    username: String,
    password: String,
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

    match Accounts::find()
        .filter(accounts::Column::Name.eq(payload.username))
        .all(&db)
        .await
    {
        Ok(results) => match results.first() {
            None => {
                let account = ActiveModel {
                    id: Default::default(),
                    name: Default::default(),
                    public_key: Default::default(),
                    private_key: Default::default(),
                    password: Default::default(),
                };
            }
            Some(_) => {
                return (
                    StatusCode::CONFLICT,
                    Json(json!({
                        "error": "an account with this name already exists"
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

    // add more checks and database insert

    (
        StatusCode::OK,
        Json(json!({
            "success": "added account to registry"
        })),
    )
}
