use crate::application::feed::error::FeedError;
use crate::domain::feed::repository::FeedRepository;
use uuid::Uuid;
pub struct DeleteFeedUseCase<R> {
    repository: R,
}
impl<R: FeedRepository> DeleteFeedUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub fn execute(&self, id: Uuid) -> Result<(), FeedError> {
        self.repository.find_by_id(id)?.ok_or(FeedError::NotFound)?;
        self.repository.delete(id).map_err(FeedError::from)
    }
}
