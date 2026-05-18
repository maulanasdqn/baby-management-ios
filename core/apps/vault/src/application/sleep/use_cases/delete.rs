use crate::application::sleep::error::SleepError;
use crate::domain::sleep::repository::SleepRepository;
use uuid::Uuid;
pub struct DeleteSleepUseCase<R> {
    repository: R,
}
impl<R: SleepRepository> DeleteSleepUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub fn execute(&self, id: Uuid) -> Result<(), SleepError> {
        self.repository.find_by_id(id)?.ok_or(SleepError::NotFound)?;
        self.repository.delete(id).map_err(SleepError::from)
    }
}
