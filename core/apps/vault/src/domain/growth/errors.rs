use std::fmt;
#[derive(Debug)]
pub enum GrowthDomainError {
    NoMeasurementProvided,
}
impl fmt::Display for GrowthDomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoMeasurementProvided => {
                write!(f, "at least one measurement (weight or height) must be provided")
            }
        }
    }
}
impl std::error::Error for GrowthDomainError {}
