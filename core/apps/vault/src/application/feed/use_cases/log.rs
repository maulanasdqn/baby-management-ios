use crate::application::feed::error::FeedError;
use crate::domain::feed::entity::{FeedLog, FeedType, NewFeedLog};
use crate::domain::feed::repository::FeedRepository;
use chrono::{DateTime, Utc};
use tracing::info;
use uuid::Uuid;
pub struct LogFeedCommand {
    pub feed_type: FeedType,
    pub amount_ml: Option<u32>,
    pub duration_minutes: Option<u32>,
    pub side: Option<String>,
    pub notes: String,
    pub logged_at: DateTime<Utc>,
}
pub struct LogFeedUseCase<R> {
    repository: R,
}
impl<R: FeedRepository> LogFeedUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub fn execute(&self, cmd: LogFeedCommand) -> Result<FeedLog, FeedError> {
        let id = Uuid::new_v4();
        let log = self
            .repository
            .create(NewFeedLog {
                id,
                feed_type: cmd.feed_type,
                amount_ml: cmd.amount_ml,
                duration_minutes: cmd.duration_minutes,
                side: cmd.side,
                notes: cmd.notes,
                logged_at: cmd.logged_at,
            })
            .map_err(FeedError::from)?;
        info!(feed_id = %log.id, "feed log created");
        Ok(log)
    }
}
