use std::str::FromStr;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
    pub schema: Option<String>,
}

impl DatabaseConfig {
    pub async fn get_connection(&self) -> SqlitePool {
        let conn_str = format!("sqlite://{}.db", self.database.as_deref().unwrap_or("mydb"),);

        tracing::info!("Connected to database at {}", conn_str);

        let options = SqliteConnectOptions::from_str(&conn_str)
            .unwrap()
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
            .busy_timeout(Duration::from_secs(5))
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .connect_with(options)
            .await
            .unwrap();

        pool
    }
}
