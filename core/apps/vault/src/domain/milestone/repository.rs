pub use config::error_db::RepositoryError;
use crate::domain::milestone::entity::{Milestone, NewMilestone, MilestonePatch};
use uuid::Uuid;
pub trait MilestoneRepository: Send + Sync {
    fn find_by_id(&self, id: Uuid) -> Result<Option<Milestone>, RepositoryError>;
    fn list(&self, limit: u32, offset: u32) -> Result<Vec<Milestone>, RepositoryError>;
    fn create(&self, milestone: NewMilestone) -> Result<Milestone, RepositoryError>;
    fn update(&self, id: Uuid, patch: MilestonePatch) -> Result<Milestone, RepositoryError>;
    fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
}
impl<T: MilestoneRepository + ?Sized> MilestoneRepository for &T {
    fn find_by_id(&self, id: Uuid) -> Result<Option<Milestone>, RepositoryError> {
        (**self).find_by_id(id)
    }
    fn list(&self, limit: u32, offset: u32) -> Result<Vec<Milestone>, RepositoryError> {
        (**self).list(limit, offset)
    }
    fn create(&self, milestone: NewMilestone) -> Result<Milestone, RepositoryError> {
        (**self).create(milestone)
    }
    fn update(&self, id: Uuid, patch: MilestonePatch) -> Result<Milestone, RepositoryError> {
        (**self).update(id, patch)
    }
    fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        (**self).delete(id)
    }
}
