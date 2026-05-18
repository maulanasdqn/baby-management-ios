use crate::domain::growth::entity::{GrowthLog, NewGrowthLog};
use crate::domain::growth::repository::GrowthRepository;
use crate::infrastructure::repository::sqlite_pool::Pool;
use chrono::{DateTime, TimeZone, Utc};
use config::error_db::RepositoryError;
use uuid::Uuid;
#[derive(Clone)]
pub struct SqliteGrowthRepository {
    pool: Pool,
}
impl SqliteGrowthRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}
impl GrowthRepository for SqliteGrowthRepository {
    fn find_by_id(&self, id: Uuid) -> Result<Option<GrowthLog>, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare("SELECT id, weight_grams, height_mm, notes, logged_at FROM growth_logs WHERE id = ?1")
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        stmt.query_row([id.to_string()], row_to_growth_log)
            .optional()
            .map_err(|e| RepositoryError::Database(e.to_string()))
    }
    fn list_by_range(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<GrowthLog>, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare("SELECT id, weight_grams, height_mm, notes, logged_at FROM growth_logs WHERE logged_at BETWEEN ?1 AND ?2 ORDER BY logged_at ASC")
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        let rows = stmt
            .query_map([from.timestamp_millis(), to.timestamp_millis()], row_to_growth_log)
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| RepositoryError::Database(e.to_string()))
    }
    fn create(&self, g: NewGrowthLog) -> Result<GrowthLog, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        conn.execute(
            "INSERT INTO growth_logs (id, weight_grams, height_mm, notes, logged_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                g.id.to_string(),
                g.weight_grams,
                g.height_mm,
                g.notes,
                g.logged_at.timestamp_millis(),
            ],
        )
        .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(GrowthLog {
            id: g.id,
            weight_grams: g.weight_grams,
            height_mm: g.height_mm,
            notes: g.notes,
            logged_at: g.logged_at,
        })
    }
    fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        conn.execute("DELETE FROM growth_logs WHERE id = ?1", [id.to_string()])
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }
}
fn row_to_growth_log(row: &rusqlite::Row) -> rusqlite::Result<GrowthLog> {
    let id_str: String = row.get(0)?;
    let logged_ms: i64 = row.get(4)?;
    Ok(GrowthLog {
        id: Uuid::parse_str(&id_str).unwrap_or_default(),
        weight_grams: row.get(1)?,
        height_mm: row.get(2)?,
        notes: row.get(3)?,
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
