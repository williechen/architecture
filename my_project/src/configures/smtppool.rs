use lettre::SmtpTransport;
use lettre::transport::smtp::authentication::Credentials;

#[derive(Debug, serde::Deserialize)]
pub struct SmtpConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub pswd: Option<String>,
}

impl SmtpConfig {
    pub fn get_connection(&self) -> SmtpTransport {
        let creds = Credentials::new(
            self.user.clone().unwrap_or_default(),
            self.pswd.clone().unwrap_or_default(), // Gmail 要用應用程式密碼
        );

        // 建立郵件傳輸器
        SmtpTransport::relay(&self.host.as_ref().unwrap_or(&"".to_string()))
            .unwrap()
            .credentials(creds)
            .build()
    }
}
