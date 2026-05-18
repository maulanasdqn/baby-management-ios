use std::fmt;
#[derive(Debug)]
pub enum MediaDomainError {
    TitleEmpty,
}
impl fmt::Display for MediaDomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TitleEmpty => write!(f, "media title must not be empty"),
        }
    }
}
impl std::error::Error for MediaDomainError {}
