use crate::application::growth::error::GrowthError;
use crate::domain::growth::entity::GrowthLog;
use crate::domain::growth::repository::GrowthRepository;
use chrono::{DateTime, Utc};
pub struct ListGrowthByRangeCommand {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}
pub struct ListGrowthByRangeUseCase<R> {
    repository: R,
}
impl<R: GrowthRepository> ListGrowthByRangeUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub fn execute(&self, cmd: ListGrowthByRangeCommand) -> Result<Vec<GrowthLog>, GrowthError> {
        self.repository
            .list_by_range(cmd.from, cmd.to)
            .map_err(GrowthError::from)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::growth::entity::{GrowthLog, NewGrowthLog};
    use crate::domain::growth::repository::GrowthRepository;
    use chrono::{DateTime, Duration, Utc};
    use config::error_db::RepositoryError;
    use std::sync::Mutex;
    use uuid::Uuid;
    struct FakeRepo(Mutex<Vec<GrowthLog>>);
    impl GrowthRepository for FakeRepo {
        fn find_by_id(&self, id: Uuid) -> Result<Option<GrowthLog>, RepositoryError> {
            Ok(self.0.lock().unwrap().iter().find(|g| g.id == id).map(|g| GrowthLog {
                id: g.id, weight_grams: g.weight_grams, height_mm: g.height_mm,
                notes: g.notes.clone(), logged_at: g.logged_at,
            }))
        }
        fn list_by_range(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<GrowthLog>, RepositoryError> {
            Ok(self.0.lock().unwrap().iter()
                .filter(|g| g.logged_at >= from && g.logged_at <= to)
                .map(|g| GrowthLog { id: g.id, weight_grams: g.weight_grams, height_mm: g.height_mm, notes: g.notes.clone(), logged_at: g.logged_at })
                .collect())
        }
        fn create(&self, n: NewGrowthLog) -> Result<GrowthLog, RepositoryError> {
            let g = GrowthLog { id: n.id, weight_grams: n.weight_grams, height_mm: n.height_mm, notes: n.notes, logged_at: n.logged_at };
            self.0.lock().unwrap().push(GrowthLog { id: g.id, weight_grams: g.weight_grams, height_mm: g.height_mm, notes: g.notes.clone(), logged_at: g.logged_at });
            Ok(g)
        }
        fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
            self.0.lock().unwrap().retain(|g| g.id != id);
            Ok(())
        }
    }
    fn growth_log_at(offset_days: i64) -> GrowthLog {
        GrowthLog {
            id: Uuid::new_v4(),
            weight_grams: Some(3000 + offset_days as u32 * 10),
            height_mm: None,
            notes: "".into(),
            logged_at: Utc::now() - Duration::days(offset_days),
        }
    }
    #[test]
    fn list_by_range_empty_repo_returns_empty() {
        let repo = FakeRepo(Mutex::new(vec![]));
        let uc = ListGrowthByRangeUseCase::new(repo);
        let result = uc.execute(ListGrowthByRangeCommand {
            from: Utc::now() - Duration::days(30),
            to: Utc::now(),
        })
        .unwrap();
        assert!(result.is_empty());
    }
    #[test]
    fn list_by_range_returns_only_items_in_range() {
        let inside = growth_log_at(5);
        let outside = growth_log_at(40);
        let repo = FakeRepo(Mutex::new(vec![
            GrowthLog { id: inside.id, weight_grams: inside.weight_grams, height_mm: inside.height_mm, notes: inside.notes.clone(), logged_at: inside.logged_at },
            GrowthLog { id: outside.id, weight_grams: outside.weight_grams, height_mm: outside.height_mm, notes: outside.notes.clone(), logged_at: outside.logged_at },
        ]));
        let uc = ListGrowthByRangeUseCase::new(repo);
        let result = uc.execute(ListGrowthByRangeCommand {
            from: Utc::now() - Duration::days(30),
            to: Utc::now(),
        })
        .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, inside.id);
    }
}
