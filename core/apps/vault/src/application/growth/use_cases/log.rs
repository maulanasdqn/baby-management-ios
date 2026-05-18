use crate::application::growth::error::GrowthError;
use crate::domain::growth::entity::{GrowthLog, NewGrowthLog};
use crate::domain::growth::repository::GrowthRepository;
use chrono::{DateTime, Utc};
use tracing::info;
use uuid::Uuid;
pub struct LogGrowthCommand {
    pub weight_grams: Option<u32>,
    pub height_mm: Option<u32>,
    pub notes: String,
    pub logged_at: DateTime<Utc>,
}
pub struct LogGrowthUseCase<R> {
    repository: R,
}
impl<R: GrowthRepository> LogGrowthUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub fn execute(&self, cmd: LogGrowthCommand) -> Result<GrowthLog, GrowthError> {
        if cmd.weight_grams.is_none() && cmd.height_mm.is_none() {
            return Err(GrowthError::NoMeasurementProvided);
        }
        let id = Uuid::new_v4();
        let log = self
            .repository
            .create(NewGrowthLog {
                id,
                weight_grams: cmd.weight_grams,
                height_mm: cmd.height_mm,
                notes: cmd.notes,
                logged_at: cmd.logged_at,
            })
            .map_err(GrowthError::from)?;
        info!(growth_id = %log.id, "growth log created");
        Ok(log)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::growth::entity::GrowthLog;
    use crate::domain::growth::repository::GrowthRepository;
    use chrono::{DateTime, Utc};
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
        fn list_by_range(&self, _: DateTime<Utc>, _: DateTime<Utc>) -> Result<Vec<GrowthLog>, RepositoryError> {
            Ok(vec![])
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
    #[test]
    fn log_with_weight_only_succeeds() {
        let repo = FakeRepo(Mutex::new(vec![]));
        let uc = LogGrowthUseCase::new(repo);
        let result = uc.execute(LogGrowthCommand {
            weight_grams: Some(3500),
            height_mm: None,
            notes: "".into(),
            logged_at: Utc::now(),
        });
        assert!(result.is_ok());
        assert_eq!(result.unwrap().weight_grams, Some(3500));
    }
    #[test]
    fn log_with_height_only_succeeds() {
        let repo = FakeRepo(Mutex::new(vec![]));
        let uc = LogGrowthUseCase::new(repo);
        let result = uc.execute(LogGrowthCommand {
            weight_grams: None,
            height_mm: Some(510),
            notes: "3-month check".into(),
            logged_at: Utc::now(),
        });
        assert!(result.is_ok());
        assert_eq!(result.unwrap().height_mm, Some(510));
    }
    #[test]
    fn log_with_no_measurements_is_rejected() {
        let repo = FakeRepo(Mutex::new(vec![]));
        let uc = LogGrowthUseCase::new(repo);
        let result = uc.execute(LogGrowthCommand {
            weight_grams: None,
            height_mm: None,
            notes: "no data".into(),
            logged_at: Utc::now(),
        });
        assert!(matches!(result, Err(GrowthError::NoMeasurementProvided)));
    }
    #[test]
    fn log_with_both_measurements_succeeds() {
        let repo = FakeRepo(Mutex::new(vec![]));
        let uc = LogGrowthUseCase::new(repo);
        let result = uc.execute(LogGrowthCommand {
            weight_grams: Some(4000),
            height_mm: Some(540),
            notes: "6-month check".into(),
            logged_at: Utc::now(),
        });
        assert!(result.is_ok());
        let log = result.unwrap();
        assert_eq!(log.weight_grams, Some(4000));
        assert_eq!(log.height_mm, Some(540));
    }
}
