use chrono::{DateTime, Utc};
use uuid::Uuid;
pub enum DiaperType {
    Wet,
    Dirty,
    Both,
}
impl DiaperType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Wet => "Wet",
            Self::Dirty => "Dirty",
            Self::Both => "Both",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Wet" => Some(Self::Wet),
            "Dirty" => Some(Self::Dirty),
            "Both" => Some(Self::Both),
            _ => None,
        }
    }
}
pub struct DiaperLog {
    pub id: Uuid,
    pub diaper_type: DiaperType,
    pub notes: String,
    pub logged_at: DateTime<Utc>,
}
pub struct NewDiaperLog {
    pub id: Uuid,
    pub diaper_type: DiaperType,
    pub notes: String,
    pub logged_at: DateTime<Utc>,
}
