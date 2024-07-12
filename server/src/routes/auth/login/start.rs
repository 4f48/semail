use crate::common::db;
use crate::common::opaque::Default as DefaultSuite;
use crate::AppState;

use entity::accounts;
use entity::prelude::Accounts;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use opaque_ke::{CredentialRequest, ServerLogin, ServerLoginStartParameters, ServerRegistration};
use rand::rngs::OsRng;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
struct Payload {
    username: String,
    request: String,
}

pub async fn main(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
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

    let results = match Accounts::find()
        .filter(accounts::Column::Name.eq(&payload.username))
        .all(&db)
        .await
    {
        Ok(results) => results,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };

    let verifier_b64 = match results.first() {
        Some(result) => result,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "this user doesn't exist"
                })),
            )
        }
    };

    let decoded = match BASE64_STANDARD.decode(&verifier_b64.verifier) {
        Ok(decoded) => decoded,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };
    let verifier: ServerRegistration<DefaultSuite> = match bincode::deserialize(&decoded) {
        Ok(verifier) => verifier,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };

    let decoded_req = match BASE64_STANDARD.decode(payload.request) {
        Ok(decoded) => decoded,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };
    let request: CredentialRequest<DefaultSuite> = match bincode::deserialize(&decoded_req) {
        Ok(request) => request,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };

    let mut server_rng = OsRng;
    let start_result = match ServerLogin::start(
        &mut server_rng,
        &state.server_setup,
        Some(verifier),
        request,
        payload.username.as_bytes(),
        ServerLoginStartParameters::default(),
    ) {
        Ok(start_result) => start_result,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };

    let serialized = match bincode::serialize(&start_result.message) {
        Ok(serialized) => serialized,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", error)
                })),
            )
        }
    };

    (
        StatusCode::OK,
        Json(json!({
            "response": BASE64_STANDARD.encode(serialized)
        })),
    )
}
