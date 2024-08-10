use crate::common::api::{b64_decode, deserialize, error_response, parse_json, return_error};
use crate::common::db::query_user;
use crate::AppState;

use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;
use webauthn_rs::prelude::{Passkey, PublicKeyCredential};

#[derive(Deserialize)]
struct StartPayload {
    username: String,
}

pub async fn start(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let payload = match parse_json::<StartPayload>(payload).await {
        Ok(payload) => payload,
        Err(error) => return error,
    };

    let user = match query_user(&payload.username).await {
        Ok(user) => user,
        Err(error) => return error,
    };

    let decoded = match b64_decode(&user.verifier).await {
        Ok(decoded) => decoded,
        Err(error) => return error,
    };
    let deserialized = match deserialize::<Passkey>(&decoded).await {
        Ok(deserialized) => deserialized,
        Err(error) => return error,
    };

    let (rcr, auth_state) = match state.rp.start_passkey_authentication(&vec![deserialized]) {
        Ok((rcr, auth_state)) => (rcr, auth_state),
        Err(error) => return return_error(StatusCode::INTERNAL_SERVER_ERROR, error),
    };

    let flow_id = Uuid::now_v7();
    state.auth_flows.login.insert(flow_id, auth_state);

    (
        StatusCode::OK,
        Json(json!({
            "flow_id": flow_id,
            "rcr": rcr
        })),
    )
}

#[derive(Deserialize)]
struct FinishPayload {
    flow_id: Uuid,
    credential: PublicKeyCredential,
}

pub async fn finish(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let payload = match parse_json::<FinishPayload>(payload).await {
        Ok(payload) => payload,
        Err(error) => return error,
    };

    let auth_state = match state.auth_flows.login.get(&payload.flow_id) {
        Some(auth_state) => auth_state,
        None => {
            return error_response(
                StatusCode::BAD_REQUEST,
                String::from("authentication flow not found"),
            )
        }
    };

    let result = match state
        .rp
        .finish_passkey_authentication(&payload.credential, &auth_state)
    {
        Ok(result) => result,
        Err(error) => return return_error(StatusCode::BAD_REQUEST, error),
    };

    // TODO: increment credential counter later (database schema needs updating)

    (
        StatusCode::OK,
        Json(json!({
            "result": result
        })),
    )
}
