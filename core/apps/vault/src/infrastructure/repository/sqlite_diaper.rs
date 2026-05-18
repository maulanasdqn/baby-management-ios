use crate::domain::diaper::entity::{DiaperLog, DiaperType, NewDiaperLog};
use crate::domain::diaper::repository::DiaperRepository;
use crate::infrastructure::repository::sqlite_pool::Pool;
use chrono::{DateTime, TimeZone, Utc};
use config::error_db::RepositoryError;
use uuid::Uuid;
#[derive(Clone)]
pub struct SqliteDiaperRepository {
    pool: Pool,
}
impl SqliteDiaperRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}
impl DiaperRepository for SqliteDiaperRepository {
    fn find_by_id(&self, id: Uuid) -> Result<Option<DiaperLog>, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare("SELECT id, diaper_type, notes, logged_at FROM diaper_logs WHERE id = ?1")
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        stmt.query_row([id.to_string()], row_to_diaper_log)
            .optional()
            .map_err(|e| RepositoryError::Database(e.to_string()))
    }
    fn list_by_range(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<DiaperLog>, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare("SELECT id, diaper_type, notes, logged_at FROM diaper_logs WHERE logged_at BETWEEN ?1 AND ?2 ORDER BY logged_at DESC")
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        let rows = stmt
            .query_map([from.timestamp_millis(), to.timestamp_millis()], row_to_diaper_log)
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| RepositoryError::Database(e.to_string()))
    }
    fn create(&self, d: NewDiaperLog) -> Result<DiaperLog, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        conn.execute(
            "INSERT INTO diaper_logs (id, diaper_type, notes, logged_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                d.id.to_string(),
                d.diaper_type.as_str(),
                d.notes,
                d.logged_at.timestamp_millis(),
            ],
        )
        .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(DiaperLog {
            id: d.id,
            diaper_type: d.diaper_type,
            notes: d.notes,
            logged_at: d.logged_at,
        })
    }
    fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        conn.execute("DELETE FROM diaper_logs WHERE id = ?1", [id.to_string()])
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }
}
fn row_to_diaper_log(row: &rusqlite::Row) -> rusqlite::Result<DiaperLog> {
    let id_str: String = row.get(0)?;
    let type_str: String = row.get(1)?;
    let logged_ms: i64 = row.get(3)?;
    Ok(DiaperLog {
        id: Uuid::parse_str(&id_str).unwrap_or_default(),
        diaper_type: DiaperType::from_str(&type_str).unwrap_or(DiaperType::Wet),
        notes: row.get(2)?,
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
