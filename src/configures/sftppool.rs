use std::sync::Arc;

use russh::{
    ChannelId,
    client::{self, Config as RusshConfig},
};
use russh_sftp::client::SftpSession;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SftpConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub password: Option<String>,
}

impl SftpConfig {
    pub async fn get_connent(&self) -> Option<SftpSession> {
        let config = RusshConfig::default();

        let mut session = russh::client::connect(
            Arc::new(config),
            format!(
                "{}:{}",
                self.host.as_ref().unwrap_or(&"localhost".to_string()),
                self.port.unwrap_or(22),
            ),
            Client {},
        )
        .await
        .unwrap();

        if session
            .authenticate_password(
                self.user.as_ref().unwrap_or(&"root".to_string()),
                self.password.as_ref().unwrap_or(&"password".to_string()),
            )
            .await
            .unwrap()
            .success()
        {
            let channel = session.channel_open_session().await.unwrap();
            channel.request_subsystem(true, "sftp").await.unwrap();
            let sftp = SftpSession::new(channel.into_stream()).await.unwrap();
            info!("current path: {:?}", sftp.canonicalize(".").await.unwrap());
            Some(sftp)
        } else {
            None
        }
    }
}

struct Client;

impl client::Handler for Client {
    type Error = anyhow::Error;

    async fn check_server_key(
        &mut self,
        server_public_key: &russh::keys::PublicKey,
    ) -> Result<bool, Self::Error> {
        info!("check_server_key: {:?}", server_public_key);
        Ok(true)
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        _session: &mut client::Session,
    ) -> Result<(), Self::Error> {
        info!("data on channel {:?}: {}", channel, data.len());
        Ok(())
    }
}
