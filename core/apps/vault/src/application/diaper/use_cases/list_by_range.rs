use crate::application::diaper::error::DiaperError;
use crate::domain::diaper::entity::DiaperLog;
use crate::domain::diaper::repository::DiaperRepository;
use chrono::{DateTime, Utc};
pub struct ListDiaperByRangeCommand {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}
pub struct ListDiaperByRangeUseCase<R> {
    repository: R,
}
impl<R: DiaperRepository> ListDiaperByRangeUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub fn execute(&self, cmd: ListDiaperByRangeCommand) -> Result<Vec<DiaperLog>, DiaperError> {
        self.repository.list_by_range(cmd.from, cmd.to).map_err(DiaperError::from)
    }
}
