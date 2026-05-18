use serde::Serialize;
pub struct SyncStatus {
    pub pending_milestones: u32,
    pub pending_growth_logs: u32,
    pub pending_media_items: u32,
    pub last_synced_at_millis: Option<i64>,
}
#[derive(Serialize)]
pub struct SyncPayload {
    pub milestones: Vec<SyncMilestoneRecord>,
    pub growth_logs: Vec<SyncGrowthRecord>,
    pub media_metadata: Vec<SyncMediaRecord>,
}
#[derive(Serialize)]
pub struct SyncMilestoneRecord {
    pub id: String,
    pub title: String,
    pub description: String,
    pub occurred_at: i64,
    pub created_at: i64,
}
#[derive(Serialize)]
pub struct SyncGrowthRecord {
    pub id: String,
    pub weight_grams: Option<u32>,
    pub height_mm: Option<u32>,
    pub notes: String,
    pub logged_at: i64,
}
#[derive(Serialize)]
pub struct SyncMediaRecord {
    pub id: String,
    pub title: String,
    pub size_bytes: u64,
    pub created_at: i64,
}
pub struct PendingMediaFile {
    pub id: String,
    pub encrypted_path: String,
}
