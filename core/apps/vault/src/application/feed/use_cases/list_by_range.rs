use crate::application::feed::error::FeedError;
use crate::domain::feed::entity::FeedLog;
use crate::domain::feed::repository::FeedRepository;
use chrono::{DateTime, Utc};
pub struct ListFeedByRangeCommand {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}
pub struct ListFeedByRangeUseCase<R> {
    repository: R,
}
impl<R: FeedRepository> ListFeedByRangeUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub fn execute(&self, cmd: ListFeedByRangeCommand) -> Result<Vec<FeedLog>, FeedError> {
        self.repository.list_by_range(cmd.from, cmd.to).map_err(FeedError::from)
    }
}
