use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub env: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
}

impl ServerConfig {
    pub fn app_env(&self) -> String {
        self.env.as_deref().unwrap_or("development").to_string()
    }

    pub fn address(&self) -> String {
        format!(
            "{}:{}",
            self.host.as_deref().unwrap_or("0.0.0.0"),
            self.port.unwrap_or(3000)
        )
    }
}
