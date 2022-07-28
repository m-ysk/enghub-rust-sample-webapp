use domain::{MockUserRepository, ProvideUserRepository};

#[derive(Debug, Default)]
pub struct MockContext {
    pub user_repository: MockUserRepository,
}

impl ProvideUserRepository for MockContext {
    type Repository = MockUserRepository;

    fn provide(&self) -> &MockUserRepository {
        &self.user_repository
    }
}
