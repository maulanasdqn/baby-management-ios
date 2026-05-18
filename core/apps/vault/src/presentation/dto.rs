#[derive(uniffi::Record)]
pub struct MilestoneDto {
    pub id: String,
    pub title: String,
    pub description: String,
    pub occurred_at_millis: i64,
    pub created_at_millis: i64,
}
#[derive(uniffi::Record)]
pub struct GrowthLogDto {
    pub id: String,
    pub weight_grams: Option<u32>,
    pub height_mm: Option<u32>,
    pub notes: String,
    pub logged_at_millis: i64,
}
#[derive(uniffi::Record)]
pub struct MediaItemDto {
    pub id: String,
    pub title: String,
    pub encrypted_path: String,
    pub size_bytes: u64,
    pub created_at_millis: i64,
}
#[derive(uniffi::Record)]
pub struct SyncStatusDto {
    pub pending_milestones: u32,
    pub pending_growth_logs: u32,
    pub pending_media_items: u32,
    pub last_synced_at_millis: Option<i64>,
    pub is_configured: bool,
}
#[derive(uniffi::Enum)]
pub enum FeedTypeDto {
    Breast,
    Bottle,
    Solid,
}
#[derive(uniffi::Record)]
pub struct FeedLogDto {
    pub id: String,
    pub feed_type: FeedTypeDto,
    pub amount_ml: Option<u32>,
    pub duration_minutes: Option<u32>,
    pub side: Option<String>,
    pub notes: String,
    pub logged_at_millis: i64,
}
#[derive(uniffi::Record)]
pub struct SleepLogDto {
    pub id: String,
    pub start_time_millis: i64,
    pub end_time_millis: i64,
    pub notes: String,
    pub duration_minutes: i64,
}
#[derive(uniffi::Enum)]
pub enum DiaperTypeDto {
    Wet,
    Dirty,
    Both,
}
#[derive(uniffi::Record)]
pub struct DiaperLogDto {
    pub id: String,
    pub diaper_type: DiaperTypeDto,
    pub notes: String,
    pub logged_at_millis: i64,
}
