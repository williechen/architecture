use std::collections::HashMap;

use lettre::{SmtpTransport, transport::smtp::authentication::Credentials};

#[derive(Debug, serde::Deserialize)]
pub struct SmtpConfig;

impl SmtpConfig {
    pub fn get_connection(opt: &HashMap<String, String>) -> SmtpTransport {
        let creds = Credentials::new(
            opt.get("user").cloned().unwrap_or_default(),
            opt.get("pswd").cloned().unwrap_or_default(), // Gmail 要用應用程式密碼
        );

        // 建立郵件傳輸器
        SmtpTransport::relay(&opt.get("host").cloned().unwrap_or_default())
            .unwrap()
            .credentials(creds)
            .build()
    }
}
