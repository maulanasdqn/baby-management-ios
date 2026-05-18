use crate::application::milestone::error::MilestoneError;
use crate::domain::milestone::entity::Milestone;
use crate::domain::milestone::repository::MilestoneRepository;
use uuid::Uuid;
pub struct DetailMilestoneUseCase<R> {
    repository: R,
}
impl<R: MilestoneRepository> DetailMilestoneUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub fn execute(&self, id: Uuid) -> Result<Milestone, MilestoneError> {
        self.repository
            .find_by_id(id)?
            .ok_or(MilestoneError::NotFound)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::milestone::entity::{MilestonePatch, NewMilestone};
    use crate::domain::milestone::repository::MilestoneRepository;
    use chrono::Utc;
    use config::error_db::RepositoryError;
    use std::sync::Mutex;
    struct FakeRepo(Mutex<Vec<Milestone>>);
    impl MilestoneRepository for FakeRepo {
        fn find_by_id(&self, id: Uuid) -> Result<Option<Milestone>, RepositoryError> {
            Ok(self.0.lock().unwrap().iter().find(|m| m.id == id).map(|m| Milestone {
                id: m.id, title: m.title.clone(), description: m.description.clone(),
                occurred_at: m.occurred_at, created_at: m.created_at,
            }))
        }
        fn list(&self, _: u32, _: u32) -> Result<Vec<Milestone>, RepositoryError> { Ok(vec![]) }
        fn create(&self, n: NewMilestone) -> Result<Milestone, RepositoryError> {
            let m = Milestone { id: n.id, title: n.title, description: n.description, occurred_at: n.occurred_at, created_at: Utc::now() };
            self.0.lock().unwrap().push(Milestone { id: m.id, title: m.title.clone(), description: m.description.clone(), occurred_at: m.occurred_at, created_at: m.created_at });
            Ok(m)
        }
        fn update(&self, _: Uuid, _: MilestonePatch) -> Result<Milestone, RepositoryError> {
            Err(RepositoryError::NotFound)
        }
        fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
            self.0.lock().unwrap().retain(|m| m.id != id);
            Ok(())
        }
    }
    #[test]
    fn get_existing_milestone_returns_it() {
        let id = Uuid::new_v4();
        let repo = FakeRepo(Mutex::new(vec![Milestone {
            id,
            title: "Crawling".into(),
            description: "Started crawling today".into(),
            occurred_at: Utc::now() - chrono::Duration::days(2),
            created_at: Utc::now(),
        }]));
        let uc = DetailMilestoneUseCase::new(repo);
        let result = uc.execute(id).unwrap();
        assert_eq!(result.id, id);
        assert_eq!(result.title, "Crawling");
    }
    #[test]
    fn get_nonexistent_milestone_returns_not_found() {
        let repo = FakeRepo(Mutex::new(vec![]));
        let uc = DetailMilestoneUseCase::new(repo);
        let result = uc.execute(Uuid::new_v4());
        assert!(matches!(result, Err(MilestoneError::NotFound)));
    }
}
