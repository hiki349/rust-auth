use anyhow::{Result};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

pub fn generate_jwt(id: i64) -> Result<String, jwt::Error> {
    let secret: Hmac<Sha256> = Hmac::new_from_slice(b"secret")?;
    let claims = BTreeMap::from([("id", id)]);
    let token = claims.sign_with_key(&secret)?;

    Ok(token)
}

pub fn verify_jwt(token: String) -> Result<i64, jwt::Error> {
    let secret: Hmac<Sha256> = Hmac::new_from_slice(b"secret")?;
    let claims: BTreeMap<String, i64> = token.verify_with_key(&secret)?;

    Ok(claims["id"])
}
