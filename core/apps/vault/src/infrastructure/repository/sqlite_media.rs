use crate::domain::media::entity::{MediaItem, NewMediaItem};
use crate::domain::media::repository::MediaRepository;
use crate::infrastructure::repository::sqlite_pool::Pool;
use chrono::{TimeZone, Utc};
use config::error_db::RepositoryError;
use uuid::Uuid;
#[derive(Clone)]
pub struct SqliteMediaRepository {
    pool: Pool,
}
impl SqliteMediaRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}
impl MediaRepository for SqliteMediaRepository {
    fn find_by_id(&self, id: Uuid) -> Result<Option<MediaItem>, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare("SELECT id, title, encrypted_path, size_bytes, created_at FROM media_metadata WHERE id = ?1")
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        stmt.query_row([id.to_string()], row_to_media_item)
            .optional()
            .map_err(|e| RepositoryError::Database(e.to_string()))
    }
    fn list(&self, limit: u32, offset: u32) -> Result<Vec<MediaItem>, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare("SELECT id, title, encrypted_path, size_bytes, created_at FROM media_metadata ORDER BY created_at DESC LIMIT ?1 OFFSET ?2")
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        let rows = stmt
            .query_map([limit, offset], row_to_media_item)
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| RepositoryError::Database(e.to_string()))
    }
    fn create(&self, m: NewMediaItem) -> Result<MediaItem, RepositoryError> {
        let now = Utc::now();
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        conn.execute(
            "INSERT INTO media_metadata (id, title, encrypted_path, size_bytes, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                m.id.to_string(),
                m.title,
                m.encrypted_path,
                m.size_bytes as i64,
                now.timestamp_millis(),
            ],
        )
        .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(MediaItem {
            id: m.id,
            title: m.title,
            encrypted_path: m.encrypted_path,
            size_bytes: m.size_bytes,
            created_at: now,
        })
    }
    fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        conn.execute("DELETE FROM media_metadata WHERE id = ?1", [id.to_string()])
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }
}
fn row_to_media_item(row: &rusqlite::Row) -> rusqlite::Result<MediaItem> {
    let id_str: String = row.get(0)?;
    let size_bytes: i64 = row.get(3)?;
    let created_ms: i64 = row.get(4)?;
    Ok(MediaItem {
        id: Uuid::parse_str(&id_str).unwrap_or_default(),
        title: row.get(1)?,
        encrypted_path: row.get(2)?,
        size_bytes: size_bytes as u64,
        created_at: Utc.timestamp_millis_opt(created_ms).single().unwrap_or_default(),
    })
}
trait OptionalExt<T> {
    fn optional(self) -> rusqlite::Result<Option<T>>;
}
impl<T> OptionalExt<T> for rusqlite::Result<T> {
    fn optional(self) -> rusqlite::Result<Option<T>> {
        match self {
            Ok(v) => Ok(Some(v)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
