pub struct SmtpConfig {
    pub server: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl SmtpConfig {
    pub fn new() -> Self {
        SmtpConfig {
            server: None,
            port: None,
            username: None,
            password: None,
        }
    }
}
