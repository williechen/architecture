use chrono::Duration;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode, errors,
    get_current_timestamp,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::configures;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    jti: String,
    sub: String,
    aud: String,
    iss: String,
    iat: u64,
    exp: u64,
}

#[derive(Debug)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: Duration,
    pub audience: String,
    pub issuer: String,
}

impl Default for JwtConfig {
    fn default() -> Self {
        JwtConfig {
            secret: configures::get_config().secret.jwt_key(),
            expiration: Duration::seconds(3600),
            audience: String::from("default_audience"),
            issuer: String::from("default_issuer"),
        }
    }
}

#[derive(Clone)]
pub struct JWT {
    encode_secret: EncodingKey,
    decode_secret: DecodingKey,
    headers: Header,
    validation: Validation,
    expiration: Duration,
    audience: String,
    issuer: String,
}

impl JWT {
    pub fn new(config: JwtConfig) -> Self {
        let encode_secret = EncodingKey::from_secret(config.secret.as_bytes());
        let decode_secret = DecodingKey::from_secret(config.secret.as_bytes());

        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[&config.audience]);
        validation.set_issuer(&[&config.issuer]);
        validation.set_required_spec_claims(&["jti", "sub", "aud", "iss", "iat", "exp"]);

        JWT {
            encode_secret,
            decode_secret,
            headers: Header::new(Algorithm::HS256),
            validation,
            expiration: config.expiration,
            audience: config.audience,
            issuer: config.issuer,
        }
    }
    pub fn encode<T>(&self, data: T) -> Result<String, errors::Error>
    where
        T: serde::Serialize,
    {
        let current_timestamp = get_current_timestamp();

        let claims = Claims {
            jti: xid::new().to_string(),
            sub: serde_json::to_string(&data).unwrap(),
            aud: self.audience.clone(),
            iss: self.issuer.clone(),
            iat: current_timestamp,
            exp: current_timestamp.saturating_add(self.expiration.num_seconds() as u64),
        };

        encode(&self.headers, &claims, &self.encode_secret)
    }

    pub fn decode<T>(&self, token: &str) -> Result<T, errors::Error>
    where
        T: DeserializeOwned,
    {
        let token_data: TokenData<Claims> = decode(token, &self.decode_secret, &self.validation)?;
        let data: T = serde_json::from_str(&token_data.claims.sub)
            .map_err(|_| errors::Error::from(errors::ErrorKind::InvalidToken))?;

        Ok(data)
    }
}
