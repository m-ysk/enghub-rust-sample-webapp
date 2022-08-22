use anyhow::{self, Context};

use domain::{ProvideUserRepository, User, UserId, UserRepository};
use error::AppError;

pub async fn get_users_by_ids<T>(ctx: &T, ids: Vec<UserId>) -> anyhow::Result<Vec<User>>
where
    T: ProvideUserRepository,
{
    let user_repository = ProvideUserRepository::provide(ctx);

    let users = user_repository
        .get_by_ids(&ids)
        .await
        .with_context(|| AppError::Internal("failed to get users".to_string()))?;

    Ok(users)
}
