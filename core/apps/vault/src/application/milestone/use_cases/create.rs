use crate::application::milestone::error::MilestoneError;
use crate::domain::milestone::entity::{Milestone, NewMilestone};
use crate::domain::milestone::repository::MilestoneRepository;
use chrono::{DateTime, Utc};
use tracing::info;
use uuid::Uuid;
pub struct CreateMilestoneCommand {
    pub title: String,
    pub description: String,
    pub occurred_at: DateTime<Utc>,
}
pub struct CreateMilestoneUseCase<R> {
    repository: R,
}
impl<R: MilestoneRepository> CreateMilestoneUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    pub fn execute(&self, cmd: CreateMilestoneCommand) -> Result<Milestone, MilestoneError> {
        if cmd.title.trim().is_empty() {
            return Err(MilestoneError::InvalidTitle);
        }
        if cmd.occurred_at > Utc::now() {
            return Err(MilestoneError::InvalidOccurredAt);
        }
        let id = Uuid::new_v4();
        let milestone = self
            .repository
            .create(NewMilestone {
                id,
                title: cmd.title,
                description: cmd.description,
                occurred_at: cmd.occurred_at,
            })
            .map_err(MilestoneError::from)?;
        info!(milestone_id = %milestone.id, "milestone created");
        Ok(milestone)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::milestone::entity::MilestonePatch;
    use crate::domain::milestone::repository::MilestoneRepository;
    use config::error_db::RepositoryError;
    use std::sync::Mutex;
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
        fn list(&self, _limit: u32, _offset: u32) -> Result<Vec<Milestone>, RepositoryError> {
            Ok(vec![])
        }
        fn create(&self, n: NewMilestone) -> Result<Milestone, RepositoryError> {
            let m = Milestone {
                id: n.id,
                title: n.title,
                description: n.description,
                occurred_at: n.occurred_at,
                created_at: Utc::now(),
            };
            self.0.lock().unwrap().push(Milestone {
                id: m.id,
                title: m.title.clone(),
                description: m.description.clone(),
                occurred_at: m.occurred_at,
                created_at: m.created_at,
            });
            Ok(m)
        }
        fn update(&self, _id: Uuid, _patch: MilestonePatch) -> Result<Milestone, RepositoryError> {
            Err(RepositoryError::NotFound)
        }
        fn delete(&self, _id: Uuid) -> Result<(), RepositoryError> {
            Ok(())
        }
    }
    #[test]
    fn create_valid_milestone() {
        let repo = FakeRepo(Mutex::new(vec![]));
        let uc = CreateMilestoneUseCase::new(repo);
        let result = uc.execute(CreateMilestoneCommand {
            title: "First Steps".into(),
            description: "Baby walked!".into(),
            occurred_at: Utc::now() - chrono::Duration::days(1),
        });
        assert!(result.is_ok());
        assert_eq!(result.unwrap().title, "First Steps");
    }
    #[test]
    fn create_milestone_empty_title_is_rejected() {
        let repo = FakeRepo(Mutex::new(vec![]));
        let uc = CreateMilestoneUseCase::new(repo);
        let result = uc.execute(CreateMilestoneCommand {
            title: "  ".into(),
            description: "".into(),
            occurred_at: Utc::now() - chrono::Duration::days(1),
        });
        assert!(matches!(result, Err(MilestoneError::InvalidTitle)));
    }
    #[test]
    fn create_milestone_future_occurred_at_is_rejected() {
        let repo = FakeRepo(Mutex::new(vec![]));
        let uc = CreateMilestoneUseCase::new(repo);
        let result = uc.execute(CreateMilestoneCommand {
            title: "Future Event".into(),
            description: "".into(),
            occurred_at: Utc::now() + chrono::Duration::days(1),
        });
        assert!(matches!(result, Err(MilestoneError::InvalidOccurredAt)));
    }
    #[test]
    fn created_milestone_is_stored_in_repo() {
        let repo = FakeRepo(Mutex::new(vec![]));
        let uc = CreateMilestoneUseCase::new(&repo);
        uc.execute(CreateMilestoneCommand {
            title: "Smile".into(),
            description: "First smile".into(),
            occurred_at: Utc::now() - chrono::Duration::days(7),
        })
        .unwrap();
        assert_eq!(repo.0.lock().unwrap().len(), 1);
    }
}
