use crate::application::diaper::error::DiaperError;
use crate::domain::diaper::entity::{DiaperLog, DiaperType, NewDiaperLog};
use crate::domain::diaper::repository::DiaperRepository;
use chrono::{DateTime, Utc};
use tracing::info;
use uuid::Uuid;
pub struct LogDiaperCommand {
    pub diaper_type: DiaperType,
    pub notes: String,
    pub logged_at: DateTime<Utc>,
}
pub struct LogDiaperUseCase<R> {
    repository: R,
}
impl<R: DiaperRepository> LogDiaperUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub fn execute(&self, cmd: LogDiaperCommand) -> Result<DiaperLog, DiaperError> {
        let id = Uuid::new_v4();
        let log = self
            .repository
            .create(NewDiaperLog {
                id,
                diaper_type: cmd.diaper_type,
                notes: cmd.notes,
                logged_at: cmd.logged_at,
            })
            .map_err(DiaperError::from)?;
        info!(diaper_id = %log.id, "diaper log created");
        Ok(log)
    }
}
