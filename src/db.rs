use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use sqlx::migrate::MigrateDatabase;
use sqlx::Sqlite;
use std::time::Duration;

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
pub async fn check_db(db: DatabaseConnection) {
    assert!(db.ping().await.is_ok());
    db.clone().close().await.unwrap();
    assert!(matches!(db.ping().await, Err(DbErr::ConnectionAcquire(_))));
}
