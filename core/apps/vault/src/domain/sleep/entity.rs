use chrono::{DateTime, Utc};
use uuid::Uuid;
pub struct SleepLog {
    pub id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub notes: String,
}
pub struct NewSleepLog {
    pub id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub notes: String,
}
