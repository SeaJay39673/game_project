use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> anyhow::Result<String> {
    Ok(hash(password, DEFAULT_COST)?)
}

pub fn verify_password(password: &str, hash: &str) -> anyhow::Result<bool> {
    Ok(verify(password, hash)?)
}