use std::fmt;
#[derive(Debug)]
pub enum MilestoneDomainError {
    TitleEmpty,
    OccurredAtInFuture,
}
impl fmt::Display for MilestoneDomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TitleEmpty => write!(f, "milestone title must not be empty"),
            Self::OccurredAtInFuture => write!(f, "occurred_at must not be in the future"),
        }
    }
}
impl std::error::Error for MilestoneDomainError {}
