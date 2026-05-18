use std::fmt;
#[derive(Debug)]
pub enum SleepDomainError {
    EndBeforeStart,
}
impl fmt::Display for SleepDomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EndBeforeStart => write!(f, "end_time must be after start_time"),
        }
    }
}
impl std::error::Error for SleepDomainError {}
