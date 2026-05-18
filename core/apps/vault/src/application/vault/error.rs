use crate::domain::vault::errors::CryptoError;
use std::fmt;
#[derive(Debug)]
pub enum VaultError {
    AlreadyInitialized,
    NotInitialized,
    KeyDerivationFailed(String),
    CryptoFailed(String),
    Internal(String),
}
impl fmt::Display for VaultError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyInitialized => write!(f, "vault already initialized"),
            Self::NotInitialized => write!(f, "vault not initialized"),
            Self::KeyDerivationFailed(m) => write!(f, "key derivation failed: {m}"),
            Self::CryptoFailed(m) => write!(f, "crypto operation failed: {m}"),
            Self::Internal(m) => write!(f, "internal error: {m}"),
        }
    }
}
impl std::error::Error for VaultError {}
impl From<CryptoError> for VaultError {
    fn from(e: CryptoError) -> Self {
        Self::CryptoFailed(e.to_string())
    }
}
