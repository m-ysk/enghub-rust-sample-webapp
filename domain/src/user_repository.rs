use anyhow;
use async_trait::async_trait;

use crate::{User, UserId};

#[async_trait]
pub trait UserRepository {
    async fn save(&self, user: &User) -> anyhow::Result<()>;
    async fn get_by_ids(&self, ids: &[UserId]) -> anyhow::Result<Vec<User>>;
}

pub trait ProvideUserRepository {
    type Repository: UserRepository;

    fn provide(&self) -> &Self::Repository;
}
