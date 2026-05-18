use std::fmt;
#[derive(Debug)]
pub enum SyncDomainError {
    InvalidServerUrl,
}
impl fmt::Display for SyncDomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidServerUrl => write!(f, "invalid server URL"),
        }
    }
}
impl std::error::Error for SyncDomainError {}
