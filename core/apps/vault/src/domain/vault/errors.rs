use std::fmt;
#[derive(Debug)]
pub enum CryptoError {
    EncryptionFailed(String),
    DecryptionFailed(String),
    InvalidKey(String),
    Io(String),
}
impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EncryptionFailed(m) => write!(f, "encryption failed: {m}"),
            Self::DecryptionFailed(m) => write!(f, "decryption failed: {m}"),
            Self::InvalidKey(m) => write!(f, "invalid key: {m}"),
            Self::Io(m) => write!(f, "io error: {m}"),
        }
    }
}
impl std::error::Error for CryptoError {}
