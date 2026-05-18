use crate::application::media::error::MediaError;
use crate::domain::media::entity::MediaItem;
use crate::domain::media::repository::MediaRepository;
pub struct ListMediaMetadataUseCase<R> {
    repository: R,
}
impl<R: MediaRepository> ListMediaMetadataUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub fn execute(&self, limit: u32, offset: u32) -> Result<Vec<MediaItem>, MediaError> {
        self.repository.list(limit, offset).map_err(MediaError::from)
    }
}
