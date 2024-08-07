use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use webauthn_rs::prelude::*;

use crate::common::{
    api::{parse_json, return_error},
    state::AppState,
};

#[derive(Deserialize)]
struct Payload {
    username: String,
}

pub async fn main(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let payload = match parse_json::<Payload>(payload).await {
        Ok(payload) => payload,
        Err(error) => return error,
    };

    let uuid = Uuid::now_v7();
    let res = match state.webauthn.start_passkey_registration(
        uuid,
        &payload.username,
        &payload.username,
        None,
    ) {
        Ok((ccr, reg_state)) => {
            state.flows.register.insert(uuid, reg_state);
            Json(ccr)
        }
        Err(error) => return return_error(StatusCode::INTERNAL_SERVER_ERROR, error),
    };

    (
        StatusCode::OK,
        Json(json!({
            "uuid": uuid,
            "result": *res
        })),
    )
}
