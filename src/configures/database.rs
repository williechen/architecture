use rbatis::RBatis;
use rbdc_sqlite::driver::SqliteDriver;
use serde::{Deserialize, Serialize};

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
    pub async fn get_connection(&self) -> RBatis {
        let conn_str = format!("sqlite://{}.db", self.database.as_deref().unwrap_or("mydb"),);
        let rbatis = RBatis::new();
        rbatis.link(SqliteDriver {}, &conn_str).await.unwrap();

        tracing::info!("Connected to database at {}", conn_str);

        rbatis
    }
}
