use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    id: Option<String>,
    pub web_permissions: Option<Vec<String>>,
    pub api_permissions: Option<Vec<String>>,
    pub is_active: Option<bool>,
    pub is_superuser: Option<bool>,
    pub last_login: Option<DateTime<Utc>>,
}

impl Default for Permission {
    fn default() -> Self {
        Permission {
            id: None,
            web_permissions: None,
            api_permissions: None,
            is_active: Some(false),
            is_superuser: Some(false),
            last_login: Some(Utc::now()),
        }
    }
}

impl Permission {
    pub fn build(&self, _db: &sqlx::SqlitePool) -> Self {
        // TODO Permissions

        Permission {
            id: self.id.clone(),
            web_permissions: Some(vec!["/".to_string()]),
            api_permissions: Some(vec!["/".to_string()]),
            is_active: self.is_active,
            is_superuser: self.is_superuser,
            last_login: Some(Utc::now()),
        }
    }

    pub fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    pub fn has_web_perm(&self, perm: &str) -> bool {
        if self.is_active.unwrap_or(false) {
            return false;
        }

        if self.is_superuser.unwrap_or(false) {
            return true;
        }

        if let Some(permissions) = &self.web_permissions {
            permissions.iter().any(|p| perm.starts_with(p))
        } else {
            false
        }
    }

    pub fn has_api_perm(&self, perm: &str) -> bool {
        if self.is_active.unwrap_or(false) {
            return false;
        }

        if self.is_superuser.unwrap_or(false) {
            return true;
        }

        if let Some(permissions) = &self.api_permissions {
            permissions.iter().any(|p| perm.starts_with(p))
        } else {
            false
        }
    }
}
