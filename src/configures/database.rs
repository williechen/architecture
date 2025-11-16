use rbatis::{RBatis, table_sync};
use rbdc_sqlite::driver::SqliteDriver;
use serde::{Deserialize, Serialize};

use crate::entities::{ssm_codemap, ssm_config, uam_user};

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
            &uam_user::UamUser::default(),
            "uam_user",
        )
        .await
        .unwrap();

        RBatis::sync(
            db,
            mapper, // Assuming UamUser implements ColumnMapper
            &ssm_codemap::SsmCodemap::default(),
            "ssm_codemap",
        )
        .await
        .unwrap();

        RBatis::sync(
            db,
            mapper, // Assuming UamUser implements ColumnMapper
            &ssm_config::SsmConfig::default(),
            "ssm_config",
        )
        .await
        .unwrap();

        tracing::info!("Database schema synchronized");
    }
}
