use config::error_db::RepositoryError;
use std::fmt;
#[derive(Debug)]
pub enum SleepError {
    NotFound,
    EndBeforeStart,
    Internal(String),
}
impl fmt::Display for SleepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "sleep log not found"),
            Self::EndBeforeStart => write!(f, "end_time must be after start_time"),
            Self::Internal(m) => write!(f, "internal error: {m}"),
        }
    }
}
impl std::error::Error for SleepError {}
impl From<RepositoryError> for SleepError {
    fn from(e: RepositoryError) -> Self {
        match e {
            RepositoryError::NotFound => Self::NotFound,
            RepositoryError::Conflict(m) | RepositoryError::Database(m) => Self::Internal(m),
        }
    }
}
