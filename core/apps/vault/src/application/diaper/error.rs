use config::error_db::RepositoryError;
use std::fmt;
#[derive(Debug)]
pub enum DiaperError {
    NotFound,
    Internal(String),
}
impl fmt::Display for DiaperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "diaper log not found"),
            Self::Internal(m) => write!(f, "internal error: {m}"),
        }
    }
}
impl std::error::Error for DiaperError {}
impl From<RepositoryError> for DiaperError {
    fn from(e: RepositoryError) -> Self {
        match e {
            RepositoryError::NotFound => Self::NotFound,
            RepositoryError::Conflict(m) | RepositoryError::Database(m) => Self::Internal(m),
        }
    }
}
