use crate::application::sync::error::SyncError;
use crate::domain::sync::entity::{SyncPayload, SyncStatus};
use crate::domain::sync::ports::SyncClient;
use crate::domain::sync::repository::SyncRepository;
use tracing::info;
pub struct SyncAllUseCase<SR, SC> {
    sync_repo: SR,
    sync_client: SC,
}
impl<SR: SyncRepository, SC: SyncClient> SyncAllUseCase<SR, SC> {
    pub fn new(sync_repo: SR, sync_client: SC) -> Self {
        Self { sync_repo, sync_client }
    }
    pub fn execute(&self, server_url: &str, api_key: &str) -> Result<SyncStatus, SyncError> {
        let milestones = self.sync_repo.get_pending_milestones()?;
        let growth_logs = self.sync_repo.get_pending_growth_logs()?;
        let media_metadata = self.sync_repo.get_pending_media_metadata()?;
        let media_files = self.sync_repo.get_pending_media_files()?;
        let milestone_ids: Vec<String> = milestones.iter().map(|m| m.id.clone()).collect();
        let growth_ids: Vec<String> = growth_logs.iter().map(|g| g.id.clone()).collect();
        let media_ids: Vec<String> = media_metadata.iter().map(|m| m.id.clone()).collect();
        let total = milestone_ids.len() + growth_ids.len() + media_ids.len();
        if total == 0 {
            info!("nothing pending — skipping push");
            return self.sync_repo.get_status().map_err(SyncError::from);
        }
        info!(
            milestones = milestone_ids.len(),
            growth_logs = growth_ids.len(),
            media_items = media_ids.len(),
            "pushing pending records",
        );
        let payload = SyncPayload { milestones, growth_logs, media_metadata };
        let synced_at = self.sync_client.push(server_url, api_key, &payload)?;
        for file in &media_files {
            let bytes = std::fs::read(&file.encrypted_path)
                .map_err(|e| SyncError::Internal(format!("read {}: {e}", file.encrypted_path)))?;
            self.sync_client.upload_media(server_url, api_key, &file.id, &bytes)?;
        }
        if !milestone_ids.is_empty() {
            self.sync_repo.mark_synced("milestone", &milestone_ids, synced_at)?;
        }
        if !growth_ids.is_empty() {
            self.sync_repo.mark_synced("growth_log", &growth_ids, synced_at)?;
        }
        if !media_ids.is_empty() {
            self.sync_repo.mark_synced("media", &media_ids, synced_at)?;
        }
        info!(synced_at, "sync push complete");
        self.sync_repo.get_status().map_err(SyncError::from)
    }
    pub fn get_status(&self) -> Result<SyncStatus, SyncError> {
        self.sync_repo.get_status().map_err(SyncError::from)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::sync::error::SyncError;
    use crate::domain::sync::entity::{PendingMediaFile, SyncGrowthRecord, SyncMediaRecord, SyncMilestoneRecord, SyncPayload, SyncStatus};
    use crate::domain::sync::ports::SyncClient;
    use crate::domain::sync::repository::SyncRepository;
    use config::error_db::RepositoryError;
    use std::sync::Mutex;
    struct FakeSyncRepo {
        milestones: Vec<SyncMilestoneRecord>,
        synced: Mutex<Vec<(String, Vec<String>, i64)>>,
    }
    impl FakeSyncRepo {
        fn new(milestones: Vec<SyncMilestoneRecord>) -> Self {
            Self { milestones, synced: Mutex::new(vec![]) }
        }
    }
    impl SyncRepository for FakeSyncRepo {
        fn get_pending_milestones(&self) -> Result<Vec<SyncMilestoneRecord>, RepositoryError> {
            Ok(self.milestones.iter().map(|m| SyncMilestoneRecord {
                id: m.id.clone(), title: m.title.clone(), description: m.description.clone(),
                occurred_at: m.occurred_at, created_at: m.created_at,
            }).collect())
        }
        fn get_pending_growth_logs(&self) -> Result<Vec<SyncGrowthRecord>, RepositoryError> { Ok(vec![]) }
        fn get_pending_media_metadata(&self) -> Result<Vec<SyncMediaRecord>, RepositoryError> { Ok(vec![]) }
        fn get_pending_media_files(&self) -> Result<Vec<PendingMediaFile>, RepositoryError> { Ok(vec![]) }
        fn mark_synced(&self, entity_type: &str, ids: &[String], synced_at_millis: i64) -> Result<(), RepositoryError> {
            self.synced.lock().unwrap().push((entity_type.into(), ids.to_vec(), synced_at_millis));
            Ok(())
        }
        fn get_last_synced_at(&self) -> Result<Option<i64>, RepositoryError> { Ok(Some(1_000_000)) }
        fn get_status(&self) -> Result<SyncStatus, RepositoryError> {
            Ok(SyncStatus { pending_milestones: 0, pending_growth_logs: 0, pending_media_items: 0, last_synced_at_millis: Some(1_000_000) })
        }
    }
    struct FakeSyncClient { pushed: Mutex<Vec<usize>> }
    impl FakeSyncClient {
        fn new() -> Self { Self { pushed: Mutex::new(vec![]) } }
    }
    impl SyncClient for FakeSyncClient {
        fn push(&self, _: &str, _: &str, payload: &SyncPayload) -> Result<i64, SyncError> {
            self.pushed.lock().unwrap().push(payload.milestones.len());
            Ok(1_000_000)
        }
        fn upload_media(&self, _: &str, _: &str, _: &str, _: &[u8]) -> Result<(), SyncError> { Ok(()) }
    }
    #[test]
    fn pushes_pending_records_and_marks_synced() {
        let repo = FakeSyncRepo::new(vec![
            SyncMilestoneRecord { id: "a".into(), title: "First Steps".into(), description: "".into(), occurred_at: 0, created_at: 0 },
        ]);
        let client = FakeSyncClient::new();
        let uc = SyncAllUseCase::new(&repo, &client);
        let status = uc.execute("http://localhost:8080", "secret").unwrap();
        assert_eq!(status.pending_milestones, 0);
        let pushed = client.pushed.lock().unwrap();
        assert_eq!(*pushed, vec![1usize]);
        let synced = repo.synced.lock().unwrap();
        assert_eq!(synced[0].0, "milestone");
    }
    #[test]
    fn empty_pending_skips_push_and_returns_status() {
        let repo = FakeSyncRepo::new(vec![]);
        let client = FakeSyncClient::new();
        let uc = SyncAllUseCase::new(&repo, &client);
        let status = uc.execute("http://localhost:8080", "secret").unwrap();
        assert_eq!(status.pending_milestones, 0);
        assert!(client.pushed.lock().unwrap().is_empty());
        assert!(repo.synced.lock().unwrap().is_empty());
    }
    #[test]
    fn get_status_returns_repo_status() {
        let repo = FakeSyncRepo::new(vec![]);
        let client = FakeSyncClient::new();
        let uc = SyncAllUseCase::new(&repo, &client);
        let status = uc.get_status().unwrap();
        assert_eq!(status.last_synced_at_millis, Some(1_000_000));
    }
}
