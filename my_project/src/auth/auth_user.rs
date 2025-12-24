use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: Option<String>,
    password: Option<String>,

    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub date_joined: Option<DateTime<Utc>>,

    pub groups: Option<Vec<String>>,
    pub is_active: Option<bool>,
    pub is_superuser: Option<bool>,
    pub last_login: Option<DateTime<Utc>>,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: None,
            username: None,
            first_name: None,
            last_name: None,
            email: None,
            password: None,
            groups: None,
            is_active: Some(false),
            is_superuser: Some(false),
            last_login: Some(Utc::now()),
            date_joined: None,
        }
    }
}

impl User {
    pub fn new(id: String) -> Self {
        User {
            id: Some(id),
            username: None,
            first_name: None,
            last_name: None,
            email: None,
            password: None,
            groups: None,
            is_active: Some(false),
            is_superuser: Some(false),
            last_login: Some(Utc::now()),
            date_joined: None,
        }
    }

    pub fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    pub fn is_authenticated(&self) -> bool {
        self.id.is_some()
    }

    pub fn is_anonymous(&self) -> bool {
        self.id.is_none()
    }

    pub fn get_full_name(&self) -> Option<String> {
        if let (Some(first), Some(last)) = (&self.first_name, &self.last_name) {
            let full_name = format!("{} {}", first, last);
            Some(full_name)
        } else {
            None
        }
    }

    pub fn set_password(&mut self, password: Option<String>) {
        self.password = password;
    }

    pub fn check_password(&self, password: &str) -> bool {
        match &self.password {
            Some(pw) => pw == password,
            None => false,
        }
    }

    pub fn has_group(&self, group: &str) -> bool {
        if self.is_active.unwrap_or(false) {
            return false;
        }

        if self.is_superuser.unwrap_or(false) {
            return true;
        }

        if let Some(groups) = &self.groups {
            groups.contains(&group.to_string())
        } else {
            false
        }
    }
}
