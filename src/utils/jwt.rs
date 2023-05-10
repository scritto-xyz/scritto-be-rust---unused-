use chrono::{Utc, Duration};
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use jsonwebtoken::errors::ErrorKind;

use crate::error::error::AppError;


#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

pub fn create() -> Result<String, AppError> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::seconds(30);
    let exp = (now + expires_in).timestamp() as usize;
    let claim: Claims = Claims {exp, iat};
    let jwt_secret = get_jwt_secret()?;
    let encoding_key: EncodingKey = EncodingKey::from_secret(jwt_secret.as_ref());

    encode(&Header::default(),&claim, &encoding_key).map_err(|_| AppError::TokenCreation)
}

pub fn is_valid(token: &str) -> Result<bool, AppError> {
    let jwt_secret = get_jwt_secret()?;
    let key: DecodingKey = DecodingKey::from_secret(jwt_secret.as_bytes());
    decode::<Claims>(&token, &key, &Validation::new(Algorithm::HS256))
        .map_err(|error| {
            match error.kind() {
                ErrorKind::ExpiredSignature => AppError::Unauthorized,
                _ => AppError::InvalidToken
            }
        })?;
    Ok(true)
}

fn get_jwt_secret() -> Result<String, AppError> {
    match env::var("JWT_SECRET") {
        Ok(s) => Ok(s),
        Err(_) => {
            tracing::debug!("JWT_SECRET env var not found");
            Err(AppError::TokenCreation)
        }
    }
}