pub use config::error_db::RepositoryError;
use crate::domain::sleep::entity::{NewSleepLog, SleepLog};
use chrono::{DateTime, Utc};
use uuid::Uuid;
pub trait SleepRepository: Send + Sync {
    fn find_by_id(&self, id: Uuid) -> Result<Option<SleepLog>, RepositoryError>;
    fn list_by_range(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<SleepLog>, RepositoryError>;
    fn create(&self, log: NewSleepLog) -> Result<SleepLog, RepositoryError>;
    fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
}
impl<T: SleepRepository + ?Sized> SleepRepository for &T {
    fn find_by_id(&self, id: Uuid) -> Result<Option<SleepLog>, RepositoryError> { (**self).find_by_id(id) }
    fn list_by_range(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<SleepLog>, RepositoryError> { (**self).list_by_range(from, to) }
    fn create(&self, log: NewSleepLog) -> Result<SleepLog, RepositoryError> { (**self).create(log) }
    fn delete(&self, id: Uuid) -> Result<(), RepositoryError> { (**self).delete(id) }
}
