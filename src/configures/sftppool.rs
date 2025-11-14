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
    pub pswd: Option<String>,
}

impl SftpConfig {
    pub async fn get_connection(&self) -> Option<SftpSession> {
        let config = RusshConfig::default();
        let default_host = "localhost".to_string();
        let host = self.host.as_ref().unwrap_or(&default_host);
        let port = self.port.unwrap_or(22);

        let default_user = "root".to_string();
        let user = self.user.as_ref().unwrap_or(&default_user);

        let default_pswd = "password".to_string();
        let pswd = self.pswd.as_ref().unwrap_or(&default_pswd);

        let mut session =
            russh::client::connect(Arc::new(config), format!("{0}:{1}", host, port,), Client {})
                .await
                .unwrap();

        if session
            .authenticate_password(user, pswd)
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
    type Error = russh::Error;

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
