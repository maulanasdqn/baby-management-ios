use crate::domain::sync::entity::{
    PendingMediaFile, SyncGrowthRecord, SyncMediaRecord, SyncMilestoneRecord, SyncStatus,
};
use crate::domain::sync::repository::SyncRepository;
use crate::infrastructure::repository::sqlite_pool::Pool;
use config::error_db::RepositoryError;
#[derive(Clone)]
pub struct SqliteSyncRepository {
    pool: Pool,
}
impl SqliteSyncRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}
impl SyncRepository for SqliteSyncRepository {
    fn get_pending_milestones(&self) -> Result<Vec<SyncMilestoneRecord>, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare(
                "SELECT id, title, description, occurred_at, created_at FROM milestones \
                 WHERE id NOT IN (SELECT entity_id FROM sync_state WHERE entity_type = 'milestone')",
            )
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        let rows = stmt
            .query_map([], |row| {
                Ok(SyncMilestoneRecord {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    occurred_at: row.get(3)?,
                    created_at: row.get(4)?,
                })
            })
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(rows)
    }
    fn get_pending_growth_logs(&self) -> Result<Vec<SyncGrowthRecord>, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare(
                "SELECT id, weight_grams, height_mm, notes, logged_at FROM growth_logs \
                 WHERE id NOT IN (SELECT entity_id FROM sync_state WHERE entity_type = 'growth_log')",
            )
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        let rows = stmt
            .query_map([], |row| {
                Ok(SyncGrowthRecord {
                    id: row.get(0)?,
                    weight_grams: row.get::<_, Option<u32>>(1)?,
                    height_mm: row.get::<_, Option<u32>>(2)?,
                    notes: row.get(3)?,
                    logged_at: row.get(4)?,
                })
            })
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(rows)
    }
    fn get_pending_media_metadata(&self) -> Result<Vec<SyncMediaRecord>, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare(
                "SELECT id, title, size_bytes, created_at FROM media_metadata \
                 WHERE id NOT IN (SELECT entity_id FROM sync_state WHERE entity_type = 'media')",
            )
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        let rows = stmt
            .query_map([], |row| {
                Ok(SyncMediaRecord {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    size_bytes: row.get::<_, u64>(2)?,
                    created_at: row.get(3)?,
                })
            })
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(rows)
    }
    fn get_pending_media_files(&self) -> Result<Vec<PendingMediaFile>, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare(
                "SELECT id, encrypted_path FROM media_metadata \
                 WHERE id NOT IN (SELECT entity_id FROM sync_state WHERE entity_type = 'media')",
            )
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        let rows = stmt
            .query_map([], |row| {
                Ok(PendingMediaFile { id: row.get(0)?, encrypted_path: row.get(1)? })
            })
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(rows)
    }
    fn mark_synced(&self, entity_type: &str, ids: &[String], synced_at_millis: i64) -> Result<(), RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        for id in ids {
            conn.execute(
                "INSERT OR REPLACE INTO sync_state (entity_type, entity_id, synced_at) VALUES (?1, ?2, ?3)",
                rusqlite::params![entity_type, id, synced_at_millis],
            )
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        }
        Ok(())
    }
    fn get_last_synced_at(&self) -> Result<Option<i64>, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        let result: Option<i64> = conn
            .query_row("SELECT MAX(synced_at) FROM sync_state", [], |row| row.get(0))
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(result)
    }
    fn get_status(&self) -> Result<SyncStatus, RepositoryError> {
        let pending_milestones = self.get_pending_milestones()?.len() as u32;
        let pending_growth_logs = self.get_pending_growth_logs()?.len() as u32;
        let pending_media_items = self.get_pending_media_metadata()?.len() as u32;
        let last_synced_at_millis = self.get_last_synced_at()?;
        Ok(SyncStatus { pending_milestones, pending_growth_logs, pending_media_items, last_synced_at_millis })
    }
}
