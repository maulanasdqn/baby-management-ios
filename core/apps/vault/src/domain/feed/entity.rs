use chrono::{DateTime, Utc};
use uuid::Uuid;
pub enum FeedType {
    Breast,
    Bottle,
    Solid,
}
impl FeedType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Breast => "Breast",
            Self::Bottle => "Bottle",
            Self::Solid => "Solid",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Breast" => Some(Self::Breast),
            "Bottle" => Some(Self::Bottle),
            "Solid" => Some(Self::Solid),
            _ => None,
        }
    }
}
pub struct FeedLog {
    pub id: Uuid,
    pub feed_type: FeedType,
    pub amount_ml: Option<u32>,
    pub duration_minutes: Option<u32>,
    pub side: Option<String>,
    pub notes: String,
    pub logged_at: DateTime<Utc>,
}
pub struct NewFeedLog {
    pub id: Uuid,
    pub feed_type: FeedType,
    pub amount_ml: Option<u32>,
    pub duration_minutes: Option<u32>,
    pub side: Option<String>,
    pub notes: String,
    pub logged_at: DateTime<Utc>,
}
