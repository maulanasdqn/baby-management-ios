use config::error_db::RepositoryError;
use std::fmt;
#[derive(Debug)]
pub enum FeedError {
    NotFound,
    InvalidFeedType,
    Internal(String),
}
impl fmt::Display for FeedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "feed log not found"),
            Self::InvalidFeedType => write!(f, "invalid feed type"),
            Self::Internal(m) => write!(f, "internal error: {m}"),
        }
    }
}
impl std::error::Error for FeedError {}
impl From<RepositoryError> for FeedError {
    fn from(e: RepositoryError) -> Self {
        match e {
            RepositoryError::NotFound => Self::NotFound,
            RepositoryError::Conflict(m) | RepositoryError::Database(m) => Self::Internal(m),
        }
    }
}
