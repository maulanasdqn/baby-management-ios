use super::entity::SyncPayload;
use crate::application::sync::error::SyncError;
pub trait SyncClient: Send + Sync {
    fn push(&self, server_url: &str, api_key: &str, payload: &SyncPayload) -> Result<i64, SyncError>;
    fn upload_media(&self, server_url: &str, api_key: &str, id: &str, encrypted_bytes: &[u8]) -> Result<(), SyncError>;
}
impl<T: SyncClient + ?Sized> SyncClient for &T {
    fn push(&self, server_url: &str, api_key: &str, payload: &SyncPayload) -> Result<i64, SyncError> {
        (**self).push(server_url, api_key, payload)
    }
    fn upload_media(&self, server_url: &str, api_key: &str, id: &str, encrypted_bytes: &[u8]) -> Result<(), SyncError> {
        (**self).upload_media(server_url, api_key, id, encrypted_bytes)
    }
}
