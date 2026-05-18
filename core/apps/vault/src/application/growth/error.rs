use config::error_db::RepositoryError;
use std::fmt;
#[derive(Debug)]
pub enum GrowthError {
    NotFound,
    NoMeasurementProvided,
    Internal(String),
}
impl fmt::Display for GrowthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "growth log not found"),
            Self::NoMeasurementProvided => {
                write!(f, "at least one measurement must be provided")
            }
            Self::Internal(m) => write!(f, "internal error: {m}"),
        }
    }
}
impl std::error::Error for GrowthError {}
impl From<RepositoryError> for GrowthError {
    fn from(e: RepositoryError) -> Self {
        match e {
            RepositoryError::NotFound => Self::NotFound,
            RepositoryError::Conflict(m) | RepositoryError::Database(m) => Self::Internal(m),
        }
    }
}
