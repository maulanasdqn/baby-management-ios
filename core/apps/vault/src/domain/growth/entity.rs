use chrono::{DateTime, Utc};
use uuid::Uuid;
pub struct GrowthLog {
    pub id: Uuid,
    pub weight_grams: Option<u32>,
    pub height_mm: Option<u32>,
    pub notes: String,
    pub logged_at: DateTime<Utc>,
}
pub struct NewGrowthLog {
    pub id: Uuid,
    pub weight_grams: Option<u32>,
    pub height_mm: Option<u32>,
    pub notes: String,
    pub logged_at: DateTime<Utc>,
}
