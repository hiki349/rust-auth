use bcrypt::{DEFAULT_COST, hash};

pub fn create_password_hash(password: &str) -> Result<String, bcrypt::BcryptError> {
    let hashed = hash(password, DEFAULT_COST)?;

    Ok(hashed)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    let is_valid = bcrypt::verify(password, hash)?;

    Ok(is_valid)
}
