use std::str::FromStr;

use anyhow::{bail, Context};
use derive_getters::Getters;
use uuid::Uuid;

use error::AppError;

#[derive(Clone, Debug, Getters)]
pub struct User {
    id: UserId,
    name: UserName,
}

impl User {
    pub fn new(name: UserName) -> User {
        User {
            id: UserId::new(),
            name,
        }
    }
}

impl User {
    /// データベースに保存されたユーザーをドメインのエンティティとして再構築します。
    pub fn reconstruct(id: String, name: String) -> anyhow::Result<User> {
        let id = id.parse().with_context(|| {
            AppError::Internal("failed to reconstruct user: invalid id".to_string())
        })?;

        let name = name.try_into().with_context(|| {
            AppError::Internal("failed to reconstruct user: invalid name".to_string())
        })?;

        Ok(User { id, name })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> UserId {
        UserId(Uuid::new_v4())
    }
}

impl FromStr for UserId {
    type Err = anyhow::Error;

    fn from_str(id: &str) -> anyhow::Result<UserId> {
        let id = id
            .parse()
            .with_context(|| AppError::InvalidArgument("invalid user id".to_string()))?;
        Ok(UserId(id))
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct UserName(String);

impl UserName {
    pub fn new(name: String) -> anyhow::Result<UserName> {
        // anyhow::ensure!を使うともっと短く書けます。
        if name.chars().any(|char| !char.is_ascii_alphanumeric()) {
            bail!(AppError::InvalidArgument(
                "username should consist of ascii alphanumerics".to_string(),
            ));
        }

        if !(2..=10).contains(&name.len()) {
            bail!(AppError::InvalidArgument(
                "username should consist of from 2 to 10 characters".to_string(),
            ));
        }

        Ok(UserName(name))
    }
}

impl TryFrom<String> for UserName {
    type Error = anyhow::Error;

    fn try_from(name: String) -> anyhow::Result<UserName> {
        UserName::new(name)
    }
}

impl std::fmt::Display for UserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("ab")]
    #[case("abcdefghij")]
    #[should_panic]
    #[case("あいう")]
    #[should_panic]
    #[case("")]
    #[should_panic]
    #[case("a")]
    #[should_panic]
    #[case("abcdefghijk")]
    #[should_panic]
    #[case("ab cd")]
    #[should_panic]
    #[case("ab\n")]
    fn test_user_name(#[case] name: &str) {
        let _ = UserName::new(name.to_string()).unwrap();
    }
}
