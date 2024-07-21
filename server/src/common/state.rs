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

use crate::common::opaque;

use dashmap::DashMap;
use opaque_ke::keypair::PrivateKey;
use opaque_ke::{Ristretto255, ServerSetup};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub server_setup: ServerSetup<opaque::Default, PrivateKey<Ristretto255>>,
    pub flows: Flows,
}

#[derive(Clone)]
pub struct Flows {
    pub(crate) register: DashMap<Uuid, String>,
    pub(crate) login: DashMap<Uuid, String>,
}
