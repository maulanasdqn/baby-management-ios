use std::fmt;
#[derive(Debug)]
pub enum DiaperDomainError {
    InvalidLoggedAt,
}
impl fmt::Display for DiaperDomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLoggedAt => write!(f, "invalid logged_at timestamp"),
        }
    }
}
impl std::error::Error for DiaperDomainError {}
