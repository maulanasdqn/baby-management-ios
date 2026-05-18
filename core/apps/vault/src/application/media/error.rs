use crate::domain::vault::errors::CryptoError;
use config::error_db::RepositoryError;
use std::fmt;
#[derive(Debug)]
pub enum MediaError {
    NotFound,
    TitleEmpty,
    EncryptionFailed(String),
    DecryptionFailed(String),
    Internal(String),
}
impl fmt::Display for MediaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "media item not found"),
            Self::TitleEmpty => write!(f, "media title must not be empty"),
            Self::EncryptionFailed(m) => write!(f, "encryption failed: {m}"),
            Self::DecryptionFailed(m) => write!(f, "decryption failed: {m}"),
            Self::Internal(m) => write!(f, "internal error: {m}"),
        }
    }
}
impl std::error::Error for MediaError {}
impl From<RepositoryError> for MediaError {
    fn from(e: RepositoryError) -> Self {
        match e {
            RepositoryError::NotFound => Self::NotFound,
            RepositoryError::Conflict(m) | RepositoryError::Database(m) => Self::Internal(m),
        }
    }
}
impl From<CryptoError> for MediaError {
    fn from(e: CryptoError) -> Self {
        match e {
            CryptoError::EncryptionFailed(m) => Self::EncryptionFailed(m),
            CryptoError::DecryptionFailed(m) => Self::DecryptionFailed(m),
            CryptoError::InvalidKey(m) | CryptoError::Io(m) => Self::Internal(m),
        }
    }
}
