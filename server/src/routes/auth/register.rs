use crate::db;
use axum::http::StatusCode;
use axum::Json;
use ed25519_dalek::pkcs8::spki::der::pem::LineEnding;
use ed25519_dalek::pkcs8::{EncodePrivateKey, EncodePublicKey};
use ed25519_dalek::SigningKey;
use entity::accounts;
use entity::accounts::ActiveModel;
use entity::prelude::Accounts;
use rand::rngs::OsRng;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    identity: String,
    salt: String,
    verifier: String,
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

    let mut csprng = OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);

    match Accounts::find()
        .filter(accounts::Column::Name.eq(&payload.identity))
        .all(&db)
        .await
    {
        Ok(results) => match results.first() {
            None => {
                let account = ActiveModel {
                    id: Set(Uuid::now_v7()),
                    name: Set(payload.identity),
                    public_key: Set(signing_key
                        .verifying_key()
                        .to_public_key_pem(LineEnding::LF)
                        .unwrap()),
                    private_key: Set(signing_key
                        .to_pkcs8_pem(LineEnding::LF)
                        .unwrap()
                        .parse()
                        .unwrap()),
                    salt: Set(payload.salt),
                    verifier: Set(payload.verifier),
                };

                match account.insert(&db).await {
                    Ok(_) => (
                        StatusCode::OK,
                        Json(json!({
                            "success": "account added to registry"
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
            Some(_) => (
                StatusCode::CONFLICT,
                Json(json!({
                    "error": "an account with this name already exists"
                })),
            ),
        },
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": format!("{}", error)
            })),
        ),
    }
}
