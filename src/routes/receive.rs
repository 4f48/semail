use crate::db;
use axum::{extract, response};
use entity::mails::ActiveModel;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbErr};
use serde_json::Value;
use uuid::Uuid;

pub async fn main(extract::Json(_payload): extract::Json<Value>) -> response::Json<Value> {
    todo!();
}

async fn insert(
    from: String,
    to: String,
    subject: Option<String>,
    body: Option<String>,
) -> Result<(), DbErr> {
    let db = match db::connect_db().await {
        Ok(database_connection) => database_connection,
        Err(db_err) => return Err(db_err),
    };

    let mail = ActiveModel {
        id: Set(Uuid::now_v7()),
        owner: Set(Uuid::now_v7()), // generate random for now, query user from username and instance IP/domain in the future
        from: Set(from),
        to: Set(to),
        subject: Set(subject),
        body: Set(body),
        ..Default::default()
    };

    match mail.insert(&db).await {
        Ok(_) => Ok(()),
        Err(db_err) => Err(db_err),
    }
}
