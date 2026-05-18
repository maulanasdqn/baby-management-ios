use std::fmt;
#[derive(Debug)]
pub enum FeedDomainError {
    InvalidLoggedAt,
}
impl fmt::Display for FeedDomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLoggedAt => write!(f, "invalid logged_at timestamp"),
        }
    }
}
impl std::error::Error for FeedDomainError {}
