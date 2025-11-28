use rbatis::{RBatis, table_sync};
use rbdc_sqlite::driver::SqliteDriver;
use serde::{Deserialize, Serialize};

use crate::entities::{allocations, batches, order_lines};

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

    pub async fn sync_schema(db: &RBatis) {
        let mapper = &table_sync::SqliteTableMapper {} as &dyn table_sync::ColumnMapper;

        RBatis::sync(
            db,
            mapper, // Assuming UamUser implements ColumnMapper
            &batches::Batch::default(),
            "batches",
        )
        .await
        .unwrap();

        RBatis::sync(
            db,
            mapper, // Assuming UamUser implements ColumnMapper
            &order_lines::OrderLine::default(),
            "order_lines",
        )
        .await
        .unwrap();

        RBatis::sync(
            db,
            mapper, // Assuming UamUser implements ColumnMapper
            &allocations::Allocation::default(),
            "allocations",
        )
        .await
        .unwrap();

        tracing::info!("Database schema synchronized");
    }
}
