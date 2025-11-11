use lettre::SmtpTransport;
use lettre::transport::smtp::authentication::Credentials;

pub struct SmtpConfig {
    pub server: Option<String>,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub password: Option<String>,
}

impl SmtpConfig {
    pub fn get_connection(&self) -> SmtpTransport {
        let creds = Credentials::new(
            self.user.clone().unwrap_or_default(),
            self.password.clone().unwrap_or_default(), // Gmail 要用應用程式密碼
        );

        // 建立郵件傳輸器
        SmtpTransport::relay(&self.server.clone().unwrap_or_default())
            .unwrap()
            .credentials(creds)
            .build()
    }
}
