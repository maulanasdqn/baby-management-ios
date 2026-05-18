use crate::application::milestone::error::MilestoneError;
use crate::domain::milestone::entity::Milestone;
use crate::domain::milestone::repository::MilestoneRepository;
pub struct ListMilestonesCommand {
    pub limit: u32,
    pub offset: u32,
}
pub struct ListMilestonesUseCase<R> {
    repository: R,
}
impl<R: MilestoneRepository> ListMilestonesUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub fn execute(&self, cmd: ListMilestonesCommand) -> Result<Vec<Milestone>, MilestoneError> {
        self.repository
            .list(cmd.limit, cmd.offset)
            .map_err(MilestoneError::from)
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
    use uuid::Uuid;
    struct FakeRepo(Mutex<Vec<Milestone>>);
    impl MilestoneRepository for FakeRepo {
        fn find_by_id(&self, id: Uuid) -> Result<Option<Milestone>, RepositoryError> {
            Ok(self.0.lock().unwrap().iter().find(|m| m.id == id).map(|m| Milestone {
                id: m.id,
                title: m.title.clone(),
                description: m.description.clone(),
                occurred_at: m.occurred_at,
                created_at: m.created_at,
            }))
        }
        fn list(&self, limit: u32, offset: u32) -> Result<Vec<Milestone>, RepositoryError> {
            let items = self.0.lock().unwrap();
            Ok(items
                .iter()
                .skip(offset as usize)
                .take(limit as usize)
                .map(|m| Milestone {
                    id: m.id,
                    title: m.title.clone(),
                    description: m.description.clone(),
                    occurred_at: m.occurred_at,
                    created_at: m.created_at,
                })
                .collect())
        }
        fn create(&self, n: NewMilestone) -> Result<Milestone, RepositoryError> {
            let m = Milestone { id: n.id, title: n.title, description: n.description, occurred_at: n.occurred_at, created_at: Utc::now() };
            self.0.lock().unwrap().push(Milestone { id: m.id, title: m.title.clone(), description: m.description.clone(), occurred_at: m.occurred_at, created_at: m.created_at });
            Ok(m)
        }
        fn update(&self, _id: Uuid, _patch: MilestonePatch) -> Result<Milestone, RepositoryError> {
            Err(RepositoryError::NotFound)
        }
        fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
            self.0.lock().unwrap().retain(|m| m.id != id);
            Ok(())
        }
    }
    fn make_milestone(title: &str) -> Milestone {
        Milestone {
            id: Uuid::new_v4(),
            title: title.into(),
            description: "".into(),
            occurred_at: Utc::now() - chrono::Duration::days(1),
            created_at: Utc::now(),
        }
    }
    #[test]
    fn list_empty_repo_returns_empty() {
        let repo = FakeRepo(Mutex::new(vec![]));
        let uc = ListMilestonesUseCase::new(&repo);
        let result = uc.execute(ListMilestonesCommand { limit: 10, offset: 0 }).unwrap();
        assert!(result.is_empty());
    }
    #[test]
    fn list_respects_limit() {
        let items = vec![make_milestone("A"), make_milestone("B"), make_milestone("C")];
        let repo = FakeRepo(Mutex::new(items));
        let uc = ListMilestonesUseCase::new(&repo);
        let result = uc.execute(ListMilestonesCommand { limit: 2, offset: 0 }).unwrap();
        assert_eq!(result.len(), 2);
    }
    #[test]
    fn list_respects_offset() {
        let items = vec![make_milestone("A"), make_milestone("B"), make_milestone("C")];
        let repo = FakeRepo(Mutex::new(items));
        let uc = ListMilestonesUseCase::new(&repo);
        let result = uc.execute(ListMilestonesCommand { limit: 10, offset: 2 }).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].title, "C");
    }
}
