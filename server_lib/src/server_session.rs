use rand::{TryRngCore, rngs::OsRng};



pub fn generate_session_token() -> anyhow::Result<String> {
    let mut bytes: [u8;32] = [0u8;32];
    OsRng.try_fill_bytes(&mut bytes)?;
    Ok(hex::encode(bytes))
}