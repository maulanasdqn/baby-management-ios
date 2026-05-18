pub use config::error_db::RepositoryError;
use crate::domain::diaper::entity::{DiaperLog, NewDiaperLog};
use chrono::{DateTime, Utc};
use uuid::Uuid;
pub trait DiaperRepository: Send + Sync {
    fn find_by_id(&self, id: Uuid) -> Result<Option<DiaperLog>, RepositoryError>;
    fn list_by_range(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<DiaperLog>, RepositoryError>;
    fn create(&self, log: NewDiaperLog) -> Result<DiaperLog, RepositoryError>;
    fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
}
impl<T: DiaperRepository + ?Sized> DiaperRepository for &T {
    fn find_by_id(&self, id: Uuid) -> Result<Option<DiaperLog>, RepositoryError> { (**self).find_by_id(id) }
    fn list_by_range(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<DiaperLog>, RepositoryError> { (**self).list_by_range(from, to) }
    fn create(&self, log: NewDiaperLog) -> Result<DiaperLog, RepositoryError> { (**self).create(log) }
    fn delete(&self, id: Uuid) -> Result<(), RepositoryError> { (**self).delete(id) }
}
