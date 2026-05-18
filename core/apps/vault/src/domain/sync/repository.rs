use super::entity::{PendingMediaFile, SyncMediaRecord, SyncMilestoneRecord, SyncGrowthRecord, SyncStatus};
use config::error_db::RepositoryError;
pub trait SyncRepository: Send + Sync {
    fn get_pending_milestones(&self) -> Result<Vec<SyncMilestoneRecord>, RepositoryError>;
    fn get_pending_growth_logs(&self) -> Result<Vec<SyncGrowthRecord>, RepositoryError>;
    fn get_pending_media_metadata(&self) -> Result<Vec<SyncMediaRecord>, RepositoryError>;
    fn get_pending_media_files(&self) -> Result<Vec<PendingMediaFile>, RepositoryError>;
    fn mark_synced(&self, entity_type: &str, ids: &[String], synced_at_millis: i64) -> Result<(), RepositoryError>;
    fn get_last_synced_at(&self) -> Result<Option<i64>, RepositoryError>;
    fn get_status(&self) -> Result<SyncStatus, RepositoryError>;
}
impl<T: SyncRepository + ?Sized> SyncRepository for &T {
    fn get_pending_milestones(&self) -> Result<Vec<SyncMilestoneRecord>, RepositoryError> {
        (**self).get_pending_milestones()
    }
    fn get_pending_growth_logs(&self) -> Result<Vec<SyncGrowthRecord>, RepositoryError> {
        (**self).get_pending_growth_logs()
    }
    fn get_pending_media_metadata(&self) -> Result<Vec<SyncMediaRecord>, RepositoryError> {
        (**self).get_pending_media_metadata()
    }
    fn get_pending_media_files(&self) -> Result<Vec<PendingMediaFile>, RepositoryError> {
        (**self).get_pending_media_files()
    }
    fn mark_synced(&self, entity_type: &str, ids: &[String], synced_at_millis: i64) -> Result<(), RepositoryError> {
        (**self).mark_synced(entity_type, ids, synced_at_millis)
    }
    fn get_last_synced_at(&self) -> Result<Option<i64>, RepositoryError> {
        (**self).get_last_synced_at()
    }
    fn get_status(&self) -> Result<SyncStatus, RepositoryError> {
        (**self).get_status()
    }
}
