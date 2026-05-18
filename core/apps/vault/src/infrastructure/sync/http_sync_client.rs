use crate::application::sync::error::SyncError;
use crate::domain::sync::entity::SyncPayload;
use crate::domain::sync::ports::SyncClient;
pub struct HttpSyncClient;
impl SyncClient for HttpSyncClient {
    fn push(&self, server_url: &str, api_key: &str, payload: &SyncPayload) -> Result<i64, SyncError> {
        let url = format!("{}/api/v1/sync/push", server_url.trim_end_matches('/'));
        let body = serde_json::to_value(payload)
            .map_err(|e| SyncError::Internal(e.to_string()))?;
        let response = ureq::post(&url)
            .set("Authorization", &format!("Bearer {api_key}"))
            .send_json(body)
            .map_err(|e| SyncError::NetworkFailed(e.to_string()))?;
        let json: serde_json::Value = response
            .into_json()
            .map_err(|e| SyncError::NetworkFailed(e.to_string()))?;
        json["synced_at"]
            .as_i64()
            .ok_or_else(|| SyncError::ServerRejected("missing synced_at in response".into()))
    }
    fn upload_media(&self, server_url: &str, api_key: &str, id: &str, encrypted_bytes: &[u8]) -> Result<(), SyncError> {
        let url = format!("{}/api/v1/sync/media/{id}", server_url.trim_end_matches('/'));
        ureq::put(&url)
            .set("Authorization", &format!("Bearer {api_key}"))
            .set("Content-Type", "application/octet-stream")
            .send_bytes(encrypted_bytes)
            .map_err(|e| SyncError::NetworkFailed(e.to_string()))?;
        Ok(())
    }
}
