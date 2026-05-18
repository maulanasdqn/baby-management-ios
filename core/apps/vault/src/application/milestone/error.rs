use crate::domain::milestone::errors::MilestoneDomainError;
use config::error_db::RepositoryError;
use std::fmt;
#[derive(Debug)]
pub enum MilestoneError {
    NotFound,
    InvalidTitle,
    InvalidOccurredAt,
    Internal(String),
}
impl fmt::Display for MilestoneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "milestone not found"),
            Self::InvalidTitle => write!(f, "milestone title must not be empty"),
            Self::InvalidOccurredAt => write!(f, "occurred_at must not be in the future"),
            Self::Internal(m) => write!(f, "internal error: {m}"),
        }
    }
}
impl std::error::Error for MilestoneError {}
impl From<RepositoryError> for MilestoneError {
    fn from(e: RepositoryError) -> Self {
        match e {
            RepositoryError::NotFound => Self::NotFound,
            RepositoryError::Conflict(m) => Self::Internal(m),
            RepositoryError::Database(m) => Self::Internal(m),
        }
    }
}
impl From<MilestoneDomainError> for MilestoneError {
    fn from(e: MilestoneDomainError) -> Self {
        match e {
            MilestoneDomainError::TitleEmpty => Self::InvalidTitle,
            MilestoneDomainError::OccurredAtInFuture => Self::InvalidOccurredAt,
        }
    }
}
