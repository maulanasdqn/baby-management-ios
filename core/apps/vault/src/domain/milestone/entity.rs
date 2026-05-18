use chrono::{DateTime, Utc};
use uuid::Uuid;
pub struct Milestone {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub occurred_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
pub struct NewMilestone {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub occurred_at: DateTime<Utc>,
}
#[derive(Default)]
pub struct MilestonePatch {
    pub title: Option<String>,
    pub description: Option<String>,
    pub occurred_at: Option<DateTime<Utc>>,
}
