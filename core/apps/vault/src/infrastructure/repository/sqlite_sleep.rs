use crate::domain::sleep::entity::{NewSleepLog, SleepLog};
use crate::domain::sleep::repository::SleepRepository;
use crate::infrastructure::repository::sqlite_pool::Pool;
use chrono::{DateTime, TimeZone, Utc};
use config::error_db::RepositoryError;
use uuid::Uuid;
#[derive(Clone)]
pub struct SqliteSleepRepository {
    pool: Pool,
}
impl SqliteSleepRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}
impl SleepRepository for SqliteSleepRepository {
    fn find_by_id(&self, id: Uuid) -> Result<Option<SleepLog>, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare("SELECT id, start_time, end_time, notes FROM sleep_logs WHERE id = ?1")
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        stmt.query_row([id.to_string()], row_to_sleep_log)
            .optional()
            .map_err(|e| RepositoryError::Database(e.to_string()))
    }
    fn list_by_range(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<SleepLog>, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare("SELECT id, start_time, end_time, notes FROM sleep_logs WHERE start_time BETWEEN ?1 AND ?2 ORDER BY start_time DESC")
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        let rows = stmt
            .query_map([from.timestamp_millis(), to.timestamp_millis()], row_to_sleep_log)
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| RepositoryError::Database(e.to_string()))
    }
    fn create(&self, s: NewSleepLog) -> Result<SleepLog, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        conn.execute(
            "INSERT INTO sleep_logs (id, start_time, end_time, notes) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                s.id.to_string(),
                s.start_time.timestamp_millis(),
                s.end_time.timestamp_millis(),
                s.notes,
            ],
        )
        .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(SleepLog {
            id: s.id,
            start_time: s.start_time,
            end_time: s.end_time,
            notes: s.notes,
        })
    }
    fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        conn.execute("DELETE FROM sleep_logs WHERE id = ?1", [id.to_string()])
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }
}
fn row_to_sleep_log(row: &rusqlite::Row) -> rusqlite::Result<SleepLog> {
    let id_str: String = row.get(0)?;
    let start_ms: i64 = row.get(1)?;
    let end_ms: i64 = row.get(2)?;
    Ok(SleepLog {
        id: Uuid::parse_str(&id_str).unwrap_or_default(),
        start_time: Utc.timestamp_millis_opt(start_ms).single().unwrap_or_default(),
        end_time: Utc.timestamp_millis_opt(end_ms).single().unwrap_or_default(),
        notes: row.get(3)?,
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
