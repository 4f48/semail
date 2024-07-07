use entity::accounts::ActiveModel;
use entity::mails;
use entity::prelude::{Accounts, Mails};
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

const DB_URL: &str = "sqlite://sqlite.db";
pub async fn create_db() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Successfully created database"),
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
        public_key: Set(String::from("abc123")),
        private_key: Set(String::from("SuperSecurePrivateKey")),
        password: Set(String::from("SuperSecurePasswordHash")),
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
