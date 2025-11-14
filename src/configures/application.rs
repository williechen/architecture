use std::sync::LazyLock;

use serde::Deserialize;

use crate::configures::database::DatabaseConfig;
use crate::configures::logger::LoggerConfig;
use crate::configures::server::ServerConfig;
use crate::configures::sftppool::SftpConfig;
use crate::configures::smtppool::SmtpConfig;

static CONFIG_FILE: LazyLock<AppConfig> = LazyLock::new(|| AppConfig::load());

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logger: LoggerConfig,
    pub sftp: SftpConfig,
    pub mail: SmtpConfig,
}

impl AppConfig {
    pub fn load() -> Self {
        config::Config::builder()
            // Add in `./Configures.toml`
            .add_source(
                config::File::with_name("Configure")
                    .format(config::FileFormat::Toml)
                    .required(true),
            )
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap()
    }
}

pub fn get_config() -> &'static AppConfig {
    &CONFIG_FILE
}
