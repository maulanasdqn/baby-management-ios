use chrono::{DateTime, Utc};
use uuid::Uuid;
pub struct MediaItem {
    pub id: Uuid,
    pub title: String,
    pub encrypted_path: String,
    pub size_bytes: u64,
    pub created_at: DateTime<Utc>,
}
pub struct NewMediaItem {
    pub id: Uuid,
    pub title: String,
    pub encrypted_path: String,
    pub size_bytes: u64,
}
