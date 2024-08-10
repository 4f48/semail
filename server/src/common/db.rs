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
use entity::accounts::{ActiveModel, Model};
use entity::prelude::{Accounts, Mails};
use entity::{accounts, mails};
use migration::{Migrator, MigratorTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectOptions, Database, DatabaseConnection, DbErr,
    EntityTrait, QueryFilter,
};
use serde_json::Value;
use sqlx::migrate::MigrateDatabase;
use sqlx::Sqlite;
use std::time::Duration;
use uuid::Uuid;

use super::api::{error_response, return_error};

const DB_URL: &str = "sqlite://sqlite.db";
pub async fn create_db() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => {
                println!("Successfully created database");
                match connect_db().await {
                    Ok(database_connection) => {
                        match Migrator::fresh(&database_connection).await {
                            Ok(_) => println!("Successfully applied migrations."),
                            Err(error) => println!("Failed to apply migrations: {}", error),
                        };
                    }
                    Err(error) => println!("Connecting to database failed: {}", error),
                };
            }
            Err(error) => panic!("Something went wrong while creating database: {}", error),
        }
    }
}

pub async fn connect_db() -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new("sqlite://sqlite.db");
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    Database::connect(opt).await
}

pub async fn query_user(username: &str) -> Result<Model, (StatusCode, Json<Value>)> {
    let db = match connect_db().await {
        Ok(database_connection) => database_connection,
        Err(error) => return Err(return_error(StatusCode::INTERNAL_SERVER_ERROR, error)),
    };

    let results = match Accounts::find()
        .filter(accounts::Column::Name.eq(username))
        .all(&db)
        .await
    {
        Ok(results) => results,
        Err(error) => return Err(return_error(StatusCode::INTERNAL_SERVER_ERROR, error)),
    };

    let user = match results.first() {
        Some(user) => user.to_owned(),
        None => {
            return Err(error_response(
                StatusCode::BAD_REQUEST,
                String::from("This user doesn't exist."),
            ))
        }
    };

    Ok(user)
}

pub async fn check_if_exists(username: &String) -> Result<bool, DbErr> {
    match Accounts::find()
        .filter(accounts::Column::Name.eq(username))
        .all(&connect_db().await?)
        .await
    {
        Ok(results) => match results.first() {
            Some(_) => Ok(true),
            None => Ok(false),
        },
        Err(error) => Err(error),
    }
}

// TEMPORARY: test database connection
pub async fn _check_db(db: DatabaseConnection) {
    assert!(db.ping().await.is_ok());
    db.clone().close().await.unwrap();
    assert!(matches!(db.ping().await, Err(DbErr::ConnectionAcquire(_))));
}

// TEMPORARY: create a user for testing
pub async fn create_test_user() -> Result<(), DbErr> {
    let db = connect_db().await?;
    let account = ActiveModel {
        id: Set(Uuid::now_v7()),
        name: Set(String::from("test")),
        verifier: Set(String::from("SuperSecurePasswordHash")),
    };

    account.insert(&db).await?;

    Ok(())
}

pub async fn get_accounts() -> Result<Vec<Value>, DbErr> {
    let db = connect_db().await?;
    let accounts: Vec<Value> = Accounts::find().into_json().all(&db).await?;

    println!("{:?}", accounts);
    Ok(accounts)
}

pub async fn get_emails(uuid: Uuid) -> Result<Vec<Value>, DbErr> {
    let db = connect_db().await?;
    let accounts: Vec<Value> = Mails::find()
        .filter(mails::Column::Owner.eq(uuid))
        .into_json()
        .all(&db)
        .await?;

    println!("{:?}", accounts);
    Ok(accounts)
}
