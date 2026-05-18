pub use config::error_db::RepositoryError;
use crate::domain::feed::entity::{FeedLog, NewFeedLog};
use chrono::{DateTime, Utc};
use uuid::Uuid;
pub trait FeedRepository: Send + Sync {
    fn find_by_id(&self, id: Uuid) -> Result<Option<FeedLog>, RepositoryError>;
    fn list_by_range(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<FeedLog>, RepositoryError>;
    fn create(&self, log: NewFeedLog) -> Result<FeedLog, RepositoryError>;
    fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
}
impl<T: FeedRepository + ?Sized> FeedRepository for &T {
    fn find_by_id(&self, id: Uuid) -> Result<Option<FeedLog>, RepositoryError> { (**self).find_by_id(id) }
    fn list_by_range(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<FeedLog>, RepositoryError> { (**self).list_by_range(from, to) }
    fn create(&self, log: NewFeedLog) -> Result<FeedLog, RepositoryError> { (**self).create(log) }
    fn delete(&self, id: Uuid) -> Result<(), RepositoryError> { (**self).delete(id) }
}
