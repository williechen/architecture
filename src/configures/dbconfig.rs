use config::Config;
use rbatis::{
    RBatis,
    table_sync::{self, ColumnMapper},
};
use rbdc_sqlite::driver::SqliteDriver;
use serde::Serialize;

pub struct DBConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
}

impl DBConfig {
    pub fn init() -> Self {
        let db_configs = Config::builder()
            // Add in `./Configures.toml`
            .add_source(config::File::with_name("Configure"))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        DBConfig {
            host: db_configs.get_string("db.host").unwrap(),
            port: db_configs.get_int("db.port").unwrap() as u16,
            username: db_configs.get_string("db.username").unwrap(),
            password: db_configs.get_string("db.password").unwrap(),
            database_name: db_configs.get_string("db.database_name").unwrap(),
        }
    }

    pub async fn get_connection(&self) -> RBatis {
        let rb = RBatis::new();
        rb.link(
            SqliteDriver {},
            &format!("sqlite://{}.db", self.database_name),
        )
        .await
        .unwrap();
        rb
    }
}

pub async fn sync_tables<T>(rb: &RBatis, model: T, table: &str)
where
    T: Serialize,
{
    let mapper = &table_sync::SqliteTableMapper {} as &dyn ColumnMapper;

    let conn = rb.acquire().await.expect("connection database fail");

    let _ = RBatis::sync(&conn, mapper, &model, table).await;
}
