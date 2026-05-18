pub use config::error_db::RepositoryError;
use crate::domain::media::entity::{MediaItem, NewMediaItem};
use uuid::Uuid;
pub trait MediaRepository: Send + Sync {
    fn find_by_id(&self, id: Uuid) -> Result<Option<MediaItem>, RepositoryError>;
    fn list(&self, limit: u32, offset: u32) -> Result<Vec<MediaItem>, RepositoryError>;
    fn create(&self, item: NewMediaItem) -> Result<MediaItem, RepositoryError>;
    fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
}
impl<T: MediaRepository + ?Sized> MediaRepository for &T {
    fn find_by_id(&self, id: Uuid) -> Result<Option<MediaItem>, RepositoryError> {
        (**self).find_by_id(id)
    }
    fn list(&self, limit: u32, offset: u32) -> Result<Vec<MediaItem>, RepositoryError> {
        (**self).list(limit, offset)
    }
    fn create(&self, item: NewMediaItem) -> Result<MediaItem, RepositoryError> {
        (**self).create(item)
    }
    fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        (**self).delete(id)
    }
}
