use password_hash::{PasswordHash, PasswordHasher, SaltString};
use pbkdf2::Pbkdf2;

const HASH_PARAMS: pbkdf2::Params = pbkdf2::Params {
    rounds: 10_000,
    output_length: 32,
};

pub fn hash_string(content: &str) -> Result<String, pbkdf2::password_hash::Error> {
    let salt = SaltString::generate(rand::thread_rng());
    let hash =
        Pbkdf2.hash_password_customized(content.as_bytes(), None, None, HASH_PARAMS, &salt)?;

    Ok(hash.serialize().to_string())
}

#[must_use]
pub fn verify_hash(content: &str, hash_str: &str) -> Option<()> {
    let hash = PasswordHash::new(hash_str).ok()?;
    hash.verify_password(&[&Pbkdf2], content).ok()
}
