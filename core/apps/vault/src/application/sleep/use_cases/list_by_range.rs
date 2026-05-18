use crate::application::sleep::error::SleepError;
use crate::domain::sleep::entity::SleepLog;
use crate::domain::sleep::repository::SleepRepository;
use chrono::{DateTime, Utc};
pub struct ListSleepByRangeCommand {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}
pub struct ListSleepByRangeUseCase<R> {
    repository: R,
}
impl<R: SleepRepository> ListSleepByRangeUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub fn execute(&self, cmd: ListSleepByRangeCommand) -> Result<Vec<SleepLog>, SleepError> {
        self.repository.list_by_range(cmd.from, cmd.to).map_err(SleepError::from)
    }
}
