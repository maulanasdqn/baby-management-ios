use config::error_db::RepositoryError;
use std::fmt;
#[derive(Debug)]
pub enum SyncError {
    NotConfigured,
    NetworkFailed(String),
    ServerRejected(String),
    Internal(String),
}
impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotConfigured => write!(f, "sync not configured — call configure_sync_server() first"),
            Self::NetworkFailed(m) => write!(f, "network error: {m}"),
            Self::ServerRejected(m) => write!(f, "server rejected sync: {m}"),
            Self::Internal(m) => write!(f, "internal sync error: {m}"),
        }
    }
}
impl std::error::Error for SyncError {}
impl From<RepositoryError> for SyncError {
    fn from(e: RepositoryError) -> Self {
        Self::Internal(e.to_string())
    }
}
