use anyhow::{self, Context};

use domain::{ProvideUserRepository, User, UserName, UserRepository};

pub struct CreateUserCommand {
    name: UserName,
}

pub async fn create_user<T>(ctx: &T, cmd: CreateUserCommand) -> anyhow::Result<User>
where
    T: ProvideUserRepository,
{
    let user = User::new(cmd.name);

    let user_repository = ProvideUserRepository::provide(ctx);

    user_repository
        .save(user.clone())
        .await
        .context("ユーザの作成に失敗しました。")?;

    Ok(user)
}
