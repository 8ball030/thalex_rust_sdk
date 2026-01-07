use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Claims {
    iat: i64,
}

pub fn make_auth_token(
    kid: &str,
    private_key_pem: String,
) -> Result<String, jsonwebtoken::errors::Error> {
    // jsonwebtoken expects iat as an integer timestamp (seconds)
    let claims = Claims {
        iat: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
    };

    let mut header = Header::new(Algorithm::RS512);
    header.kid = Some(kid.to_owned());

    let key = EncodingKey::from_rsa_pem(private_key_pem.as_bytes())?;

    encode(&header, &claims, &key)
}
