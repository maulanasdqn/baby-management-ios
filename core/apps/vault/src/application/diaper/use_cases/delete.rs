use crate::application::diaper::error::DiaperError;
use crate::domain::diaper::repository::DiaperRepository;
use uuid::Uuid;
pub struct DeleteDiaperUseCase<R> {
    repository: R,
}
impl<R: DiaperRepository> DeleteDiaperUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub fn execute(&self, id: Uuid) -> Result<(), DiaperError> {
        self.repository.find_by_id(id)?.ok_or(DiaperError::NotFound)?;
        self.repository.delete(id).map_err(DiaperError::from)
    }
}
