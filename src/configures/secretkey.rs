use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecretConfig {
    pub jwt_secret: Option<String>,
}

impl SecretConfig {
    pub fn jwt_key(&self) -> String {
        self.jwt_secret
            .as_deref()
            .unwrap_or("default_secret_key_please_change_it")
            .to_string()
    }
}
