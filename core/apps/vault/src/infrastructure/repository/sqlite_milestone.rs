use crate::domain::milestone::entity::{Milestone, MilestonePatch, NewMilestone};
use crate::domain::milestone::repository::MilestoneRepository;
use crate::infrastructure::repository::sqlite_pool::Pool;
use chrono::{TimeZone, Utc};
use config::error_db::RepositoryError;
use uuid::Uuid;
#[derive(Clone)]
pub struct SqliteMilestoneRepository {
    pool: Pool,
}
impl SqliteMilestoneRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}
impl MilestoneRepository for SqliteMilestoneRepository {
    fn find_by_id(&self, id: Uuid) -> Result<Option<Milestone>, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare(
                "SELECT id, title, description, occurred_at, created_at FROM milestones WHERE id = ?1",
            )
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        let result = stmt
            .query_row([id.to_string()], row_to_milestone)
            .optional()
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(result)
    }
    fn list(&self, limit: u32, offset: u32) -> Result<Vec<Milestone>, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare(
                "SELECT id, title, description, occurred_at, created_at FROM milestones ORDER BY occurred_at DESC LIMIT ?1 OFFSET ?2",
            )
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        let rows = stmt
            .query_map([limit, offset], row_to_milestone)
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| RepositoryError::Database(e.to_string()))
    }
    fn create(&self, m: NewMilestone) -> Result<Milestone, RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        let now = Utc::now();
        conn.execute(
            "INSERT INTO milestones (id, title, description, occurred_at, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                m.id.to_string(),
                m.title,
                m.description,
                m.occurred_at.timestamp_millis(),
                now.timestamp_millis(),
            ],
        )
        .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(Milestone {
            id: m.id,
            title: m.title,
            description: m.description,
            occurred_at: m.occurred_at,
            created_at: now,
        })
    }
    fn update(&self, id: Uuid, patch: MilestonePatch) -> Result<Milestone, RepositoryError> {
        let existing = self.find_by_id(id)?.ok_or(RepositoryError::NotFound)?;
        let title = patch.title.unwrap_or(existing.title);
        let description = patch.description.unwrap_or(existing.description);
        let occurred_at = patch.occurred_at.unwrap_or(existing.occurred_at);
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        conn.execute(
            "UPDATE milestones SET title = ?1, description = ?2, occurred_at = ?3 WHERE id = ?4",
            rusqlite::params![title, description, occurred_at.timestamp_millis(), id.to_string()],
        )
        .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(Milestone { id, title, description, occurred_at, created_at: existing.created_at })
    }
    fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let conn = self.pool.lock().map_err(|e| RepositoryError::Database(e.to_string()))?;
        conn.execute("DELETE FROM milestones WHERE id = ?1", [id.to_string()])
            .map_err(|e| RepositoryError::Database(e.to_string()))?;
        Ok(())
    }
}
fn row_to_milestone(row: &rusqlite::Row) -> rusqlite::Result<Milestone> {
    let id_str: String = row.get(0)?;
    let occurred_ms: i64 = row.get(3)?;
    let created_ms: i64 = row.get(4)?;
    Ok(Milestone {
        id: Uuid::parse_str(&id_str).unwrap_or_default(),
        title: row.get(1)?,
        description: row.get(2)?,
        occurred_at: Utc.timestamp_millis_opt(occurred_ms).single().unwrap_or_default(),
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
