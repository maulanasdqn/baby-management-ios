use crate::application::sleep::error::SleepError;
use crate::domain::sleep::entity::{NewSleepLog, SleepLog};
use crate::domain::sleep::repository::SleepRepository;
use chrono::{DateTime, Utc};
use tracing::info;
use uuid::Uuid;
pub struct LogSleepCommand {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub notes: String,
}
pub struct LogSleepUseCase<R> {
    repository: R,
}
impl<R: SleepRepository> LogSleepUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub fn execute(&self, cmd: LogSleepCommand) -> Result<SleepLog, SleepError> {
        if cmd.end_time <= cmd.start_time {
            return Err(SleepError::EndBeforeStart);
        }
        let id = Uuid::new_v4();
        let log = self
            .repository
            .create(NewSleepLog {
                id,
                start_time: cmd.start_time,
                end_time: cmd.end_time,
                notes: cmd.notes,
            })
            .map_err(SleepError::from)?;
        info!(sleep_id = %log.id, "sleep log created");
        Ok(log)
    }
}
