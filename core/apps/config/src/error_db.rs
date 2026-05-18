use std::fmt;
#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    Conflict(String),
    Database(String),
}
impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "entity not found"),
            Self::Conflict(msg) => write!(f, "conflict: {}", msg),
            Self::Database(msg) => write!(f, "database error: {}", msg),
        }
    }
}
impl std::error::Error for RepositoryError {}
