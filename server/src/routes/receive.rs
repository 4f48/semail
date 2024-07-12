/*
 * This file is part of SE-Mail.
 *
 * Copyright © 2024 Oliver Pirger <0x4f48@proton.me>
 *
 * SE-Mail is free software: you can redistribute it and/or modify it under the terms of the
 * GNU General Public License, version 3, as published by the Free Software Foundation.
 *
 * SE-Mail is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 *  without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 *  See the GNU General Public License for more details.
 *  
 * You should have received a copy of the GNU General Public License along with SE-Mail.
 * If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-only
 */

use crate::common::db;
use axum::http::StatusCode;
use axum::Json;
use dotenv::dotenv;
use entity::accounts;
use entity::mails::ActiveModel;
use entity::prelude::Accounts;
use regex::Regex;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::env;
use uuid::Uuid;

#[derive(Deserialize)]
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
                    "error": format!("{}", error)
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

    dotenv().ok();
    if recipient.instance != env::var("INSTANCE_URL").expect("INSTANCE_URL is not defined") {
        return (
            StatusCode::MISDIRECTED_REQUEST,
            Json(json!({
                "error": format!("wrong instance, this is {}, not {}", std::env::var("INSTANCE_URL").expect("INSTANCE_URL is not defined"), recipient.instance)
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
