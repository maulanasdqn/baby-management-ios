use crate::application::diaper::error::DiaperError;
use crate::application::feed::error::FeedError;
use crate::application::growth::error::GrowthError;
use crate::application::media::error::MediaError;
use crate::application::milestone::error::MilestoneError;
use crate::application::sleep::error::SleepError;
use crate::application::sync::error::SyncError;
use crate::application::vault::error::VaultError;
use std::fmt;
#[derive(Debug, uniffi::Error)]
pub enum FfiError {
    NotFound,
    Validation { msg: String },
    Crypto { msg: String },
    Internal { msg: String },
}
impl fmt::Display for FfiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "not found"),
            Self::Validation { msg } => write!(f, "validation error: {msg}"),
            Self::Crypto { msg } => write!(f, "crypto error: {msg}"),
            Self::Internal { msg } => write!(f, "internal error: {msg}"),
        }
    }
}
impl std::error::Error for FfiError {}
impl From<MilestoneError> for FfiError {
    fn from(e: MilestoneError) -> Self {
        match e {
            MilestoneError::NotFound => Self::NotFound,
            MilestoneError::InvalidTitle | MilestoneError::InvalidOccurredAt => {
                Self::Validation { msg: e.to_string() }
            }
            MilestoneError::Internal(m) => Self::Internal { msg: m },
        }
    }
}
impl From<GrowthError> for FfiError {
    fn from(e: GrowthError) -> Self {
        match e {
            GrowthError::NotFound => Self::NotFound,
            GrowthError::NoMeasurementProvided => Self::Validation { msg: e.to_string() },
            GrowthError::Internal(m) => Self::Internal { msg: m },
        }
    }
}
impl From<MediaError> for FfiError {
    fn from(e: MediaError) -> Self {
        match e {
            MediaError::NotFound => Self::NotFound,
            MediaError::TitleEmpty => Self::Validation { msg: e.to_string() },
            MediaError::EncryptionFailed(m) | MediaError::DecryptionFailed(m) => {
                Self::Crypto { msg: m }
            }
            MediaError::Internal(m) => Self::Internal { msg: m },
        }
    }
}
impl From<SyncError> for FfiError {
    fn from(e: SyncError) -> Self {
        match e {
            SyncError::NotConfigured => Self::Validation { msg: e.to_string() },
            SyncError::NetworkFailed(m) => Self::Internal { msg: m },
            SyncError::ServerRejected(m) => Self::Internal { msg: m },
            SyncError::Internal(m) => Self::Internal { msg: m },
        }
    }
}
impl From<VaultError> for FfiError {
    fn from(e: VaultError) -> Self {
        match e {
            VaultError::NotInitialized | VaultError::AlreadyInitialized => {
                Self::Validation { msg: e.to_string() }
            }
            VaultError::KeyDerivationFailed(m) | VaultError::CryptoFailed(m) => {
                Self::Crypto { msg: m }
            }
            VaultError::Internal(m) => Self::Internal { msg: m },
        }
    }
}
impl From<FeedError> for FfiError {
    fn from(e: FeedError) -> Self {
        match e {
            FeedError::NotFound => Self::NotFound,
            FeedError::InvalidFeedType => Self::Validation { msg: e.to_string() },
            FeedError::Internal(m) => Self::Internal { msg: m },
        }
    }
}
impl From<SleepError> for FfiError {
    fn from(e: SleepError) -> Self {
        match e {
            SleepError::NotFound => Self::NotFound,
            SleepError::EndBeforeStart => Self::Validation { msg: e.to_string() },
            SleepError::Internal(m) => Self::Internal { msg: m },
        }
    }
}
impl From<DiaperError> for FfiError {
    fn from(e: DiaperError) -> Self {
        match e {
            DiaperError::NotFound => Self::NotFound,
            DiaperError::Internal(m) => Self::Internal { msg: m },
        }
    }
}
