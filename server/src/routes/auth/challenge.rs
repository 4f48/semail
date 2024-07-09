use crate::db;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::Json;
use entity::accounts;
use entity::prelude::Accounts;
use sea_orm::EntityTrait;
use sea_orm::{ActiveModelTrait, ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::Sha512;
use srp::groups::G_8192;
use srp::server::SrpServer;
use std::collections::HashMap;
use ed25519_dalek::pkcs8::spki::der::pem::LineEnding;
use sea_orm::ActiveValue::Set;
use uuid::Uuid;
use entity::accounts::ActiveModel;

#[derive(Serialize, Deserialize)]
pub struct Params {
    identity: String,
}

pub async fn main(Query(params): Query<HashMap<String, String>>) -> (StatusCode, Json<Value>) {
    let params: Params = Params {
        identity: match params.get("identity") {
            Some(identity) => String::from(identity),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "identity query parameter was not provided"
                    })),
                )
            }
        },
    };

    let server = SrpServer::<Sha512>::new(&G_8192); // switch to argon2 later if possible

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

    return match Accounts::find()
        .filter(accounts::Column::Name.eq(params.identity))
        .all(&db)
        .await
    {
        Ok(results) => match results.first() {
            Some(result) => {
                let (salt, verifier) = (&result.salt, &result.verifier);
                let b = [0u8; 64];
                let B = server.compute_public_ephemeral(&b, (&verifier).as_ref());
                (
                    StatusCode::OK,
                    Json(json!({
                        "secret": format!("{:?}", b),
                        "public": format!("{:?}", B)
                    }))
                )
            },
            None => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "this user does not exist"
                }))
            )
        },
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": format!("{}", error)
            })),
        ),
    };


(StatusCode::NOT_IMPLEMENTED, Json(json!({})))
}
