use crate::domain::diaper::entity::{DiaperLog, DiaperType};
use crate::domain::feed::entity::{FeedLog, FeedType};
use crate::domain::growth::entity::GrowthLog;
use crate::domain::media::entity::MediaItem;
use crate::domain::milestone::entity::Milestone;
use crate::domain::sleep::entity::SleepLog;
use crate::presentation::dto::{
    DiaperLogDto, DiaperTypeDto, FeedLogDto, FeedTypeDto, GrowthLogDto, MediaItemDto,
    MilestoneDto, SleepLogDto,
};
pub fn milestone_to_dto(m: Milestone) -> MilestoneDto {
    MilestoneDto {
        id: m.id.to_string(),
        title: m.title,
        description: m.description,
        occurred_at_millis: m.occurred_at.timestamp_millis(),
        created_at_millis: m.created_at.timestamp_millis(),
    }
}
pub fn growth_log_to_dto(g: GrowthLog) -> GrowthLogDto {
    GrowthLogDto {
        id: g.id.to_string(),
        weight_grams: g.weight_grams,
        height_mm: g.height_mm,
        notes: g.notes,
        logged_at_millis: g.logged_at.timestamp_millis(),
    }
}
pub fn media_item_to_dto(m: MediaItem) -> MediaItemDto {
    MediaItemDto {
        id: m.id.to_string(),
        title: m.title,
        encrypted_path: m.encrypted_path,
        size_bytes: m.size_bytes,
        created_at_millis: m.created_at.timestamp_millis(),
    }
}
pub fn feed_log_to_dto(f: FeedLog) -> FeedLogDto {
    FeedLogDto {
        id: f.id.to_string(),
        feed_type: match f.feed_type {
            FeedType::Breast => FeedTypeDto::Breast,
            FeedType::Bottle => FeedTypeDto::Bottle,
            FeedType::Solid => FeedTypeDto::Solid,
        },
        amount_ml: f.amount_ml,
        duration_minutes: f.duration_minutes,
        side: f.side,
        notes: f.notes,
        logged_at_millis: f.logged_at.timestamp_millis(),
    }
}
pub fn sleep_log_to_dto(s: SleepLog) -> SleepLogDto {
    let duration_minutes = (s.end_time - s.start_time).num_minutes();
    SleepLogDto {
        id: s.id.to_string(),
        start_time_millis: s.start_time.timestamp_millis(),
        end_time_millis: s.end_time.timestamp_millis(),
        notes: s.notes,
        duration_minutes,
    }
}
pub fn diaper_log_to_dto(d: DiaperLog) -> DiaperLogDto {
    DiaperLogDto {
        id: d.id.to_string(),
        diaper_type: match d.diaper_type {
            DiaperType::Wet => DiaperTypeDto::Wet,
            DiaperType::Dirty => DiaperTypeDto::Dirty,
            DiaperType::Both => DiaperTypeDto::Both,
        },
        notes: d.notes,
        logged_at_millis: d.logged_at.timestamp_millis(),
    }
}
