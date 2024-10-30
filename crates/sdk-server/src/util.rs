use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};
use thiserror::Error;

const SDK_PRIVATE_KEY: &[u8] = include_bytes!("../rsa/private_key.der");

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("failed to decrypt: {0}")]
    Decrypt(#[from] rsa::Error),
    #[error("failed to decode base64 string")]
    FromBase64,
    #[error("from_utf8 failed: {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
}

pub fn rsa_decrypt(cipher: &str) -> Result<String, CryptoError> {
    let private_key: RsaPrivateKey = rsa::pkcs8::DecodePrivateKey::from_pkcs8_der(SDK_PRIVATE_KEY)
        .expect("failed to decode private key");
    let payload = private_key.decrypt(
        Pkcs1v15Encrypt,
        &rbase64::decode(cipher).map_err(|_| CryptoError::FromBase64)?,
    )?;

    Ok(String::from_utf8(payload)?)
}
