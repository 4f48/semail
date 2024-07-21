/*
 * This file is part of SE-Mail.
 *
 * Copyright © 2024 Oliver Pirger <0x4f48@proton.me>
 *
 * SE-Mail is free software: you can redistribute it and/or modify it under the terms of the
 * GNU General Public License, version 3, as published by the Free Software Foundation.
 *
 * SE-Mail is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with SE-Mail.
 * If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-only
 */

use axum::http::StatusCode;
use axum::Json;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

pub type ErrorResponse = (StatusCode, Json<Value>);

pub async fn parse_json<S>(payload: Value) -> Result<S, ErrorResponse>
where
    S: DeserializeOwned,
{
    match serde_json::from_value(payload) {
        Ok(payload) => Ok(payload),
        Err(error) => Err(return_error(StatusCode::BAD_REQUEST, error)),
    }
}

pub async fn b64_encode(bytes: Vec<u8>) -> String {
    BASE64_STANDARD.encode(bytes)
}

pub async fn b64_decode(base64: &String) -> Result<Vec<u8>, ErrorResponse> {
    match BASE64_STANDARD.decode(base64) {
        Ok(bytes) => Ok(bytes),
        Err(error) => Err(return_error(StatusCode::INTERNAL_SERVER_ERROR, error)),
    }
}

pub async fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>, ErrorResponse>
where
    T: serde::Serialize,
{
    match bincode::serialize(value) {
        Ok(serialized) => Ok(serialized),
        Err(error) => Err(return_error(StatusCode::INTERNAL_SERVER_ERROR, error)),
    }
}

pub async fn deserialize<'a, S>(bytes: <'a>&[u8]) -> Result<S, ErrorResponse>
where
    S: serde::de::Deserialize<'a>,
{
    match bincode::deserialize::<S>(bytes) {
        Ok(deserialized) => Ok(deserialized),
        Err(error) => Err(return_error(StatusCode::INTERNAL_SERVER_ERROR, error)),
    }
}

pub async fn encode<T: ?Sized>(value: &T) -> Result<String, ErrorResponse>
where
    T: serde::Serialize,
{
    match serialize(value).await {
        Ok(serialized) => Ok(b64_encode(serialized).await),
        Err(error) => Err(error),
    }
}

pub async fn decode<'a, S>(base64: &String) -> Result<S, ErrorResponse>
where
    S: serde::de::Deserialize<'a>,
{
    match deserialize::<S>(match b64_decode(base64).await {
        Ok(bytes) => (&bytes).to_owned(),
        Err(error) => return Err(error),
    })
    .await
    {
        Ok(message) => Ok(message),
        Err(error) => Err(error),
    }
}

pub fn return_error<E>(status: StatusCode, error: E) -> ErrorResponse
where
    E: std::fmt::Display,
{
    error_response(status, format!("{}", error))
}

pub fn error_response(status: StatusCode, error: String) -> ErrorResponse {
    (
        status,
        Json(json!({
            "error": error
        })),
    )
}
