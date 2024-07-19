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
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

pub fn parse_json<T>(payload: Value) -> Result<T, (StatusCode, Json<Value>)>
where
    T: DeserializeOwned,
{
    match serde_json::from_value(payload) {
        Ok(payload) => Ok(payload),
        Err(error) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": format!("{}", error)
            })),
        )),
    }
}
