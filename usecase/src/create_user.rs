use anyhow;

use domain::{ProvideUserRepository, User, UserName, UserRepository};

pub struct CreateUserCommand {
    name: UserName,
}

pub async fn create_user<T>(ctx: &T, cmd: CreateUserCommand) -> anyhow::Result<()>
where
    T: ProvideUserRepository,
{
    let user = User::new(cmd.name);

    let user_repository = ProvideUserRepository::provide(ctx);

    user_repository.save(user).await
}
