use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

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

        let pool_options = SqlitePoolOptions::new();

        let pool = pool_options
            .connect(&conn_str)
            .await
            .expect("Failed to create database po`ol");

        tracing::info!("Connected to database at {}", conn_str);

        pool
    }
}
