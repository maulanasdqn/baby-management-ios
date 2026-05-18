pub use config::error_db::RepositoryError;
use crate::domain::growth::entity::{GrowthLog, NewGrowthLog};
use chrono::{DateTime, Utc};
use uuid::Uuid;
pub trait GrowthRepository: Send + Sync {
    fn find_by_id(&self, id: Uuid) -> Result<Option<GrowthLog>, RepositoryError>;
    fn list_by_range(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<GrowthLog>, RepositoryError>;
    fn create(&self, log: NewGrowthLog) -> Result<GrowthLog, RepositoryError>;
    fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
}
impl<T: GrowthRepository + ?Sized> GrowthRepository for &T {
    fn find_by_id(&self, id: Uuid) -> Result<Option<GrowthLog>, RepositoryError> {
        (**self).find_by_id(id)
    }
    fn list_by_range(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<GrowthLog>, RepositoryError> {
        (**self).list_by_range(from, to)
    }
    fn create(&self, log: NewGrowthLog) -> Result<GrowthLog, RepositoryError> {
        (**self).create(log)
    }
    fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        (**self).delete(id)
    }
}
