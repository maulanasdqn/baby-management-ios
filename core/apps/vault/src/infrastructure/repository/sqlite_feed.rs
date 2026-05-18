use crate::domain::feed::entity::{FeedLog, FeedType, NewFeedLog};
use crate::domain::feed::repository::FeedRepository;
use crate::infrastructure::repository::sqlite_pool::Pool;
use chrono::{DateTime, TimeZone, Utc};
use config::error_db::RepositoryError;
use uuid::Uuid;
#[derive(Clone)]
pub struct SqliteFeedRepository {
    pool: Pool,
}
impl SqliteFeedRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}
impl FeedRepository for SqliteFeedRepository {
    fn find_by_id(&self, id: Uuid) -> Result<Option<FeedLog>, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare("SELECT id, feed_type, amount_ml, duration_minutes, side, notes, logged_at FROM feed_logs WHERE id = ?1")
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        stmt.query_row([id.to_string()], row_to_feed_log)
            .optional()
            .map_err(|e| RepositoryError::Database(e.to_string()))
    }
    fn list_by_range(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<FeedLog>, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare("SELECT id, feed_type, amount_ml, duration_minutes, side, notes, logged_at FROM feed_logs WHERE logged_at BETWEEN ?1 AND ?2 ORDER BY logged_at DESC")
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        let rows = stmt
            .query_map([from.timestamp_millis(), to.timestamp_millis()], row_to_feed_log)
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| RepositoryError::Database(e.to_string()))
    }
    fn create(&self, f: NewFeedLog) -> Result<FeedLog, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        conn.execute(
            "INSERT INTO feed_logs (id, feed_type, amount_ml, duration_minutes, side, notes, logged_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                f.id.to_string(),
                f.feed_type.as_str(),
                f.amount_ml,
                f.duration_minutes,
                f.side,
                f.notes,
                f.logged_at.timestamp_millis(),
            ],
        )
        .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(FeedLog {
            id: f.id,
            feed_type: f.feed_type,
            amount_ml: f.amount_ml,
            duration_minutes: f.duration_minutes,
            side: f.side,
            notes: f.notes,
            logged_at: f.logged_at,
        })
    }
    fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        conn.execute("DELETE FROM feed_logs WHERE id = ?1", [id.to_string()])
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }
}
fn row_to_feed_log(row: &rusqlite::Row) -> rusqlite::Result<FeedLog> {
    let id_str: String = row.get(0)?;
    let type_str: String = row.get(1)?;
    let logged_ms: i64 = row.get(6)?;
    Ok(FeedLog {
        id: Uuid::parse_str(&id_str).unwrap_or_default(),
        feed_type: FeedType::from_str(&type_str).unwrap_or(FeedType::Bottle),
        amount_ml: row.get(2)?,
        duration_minutes: row.get(3)?,
        side: row.get(4)?,
        notes: row.get(5)?,
        logged_at: Utc.timestamp_millis_opt(logged_ms).single().unwrap_or_default(),
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
