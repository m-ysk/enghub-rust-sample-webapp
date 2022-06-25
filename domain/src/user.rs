use anyhow::anyhow;
use derive_getters::Getters;
use uuid::Uuid;

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

#[derive(Clone, Copy, Debug)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> UserId {
        UserId(Uuid::new_v4())
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
        // bail!またはensure!を使うともっと簡潔に書けます。
        if !name.is_ascii() {
            return Err(anyhow!("名前はASCII文字でなければなりません: {}", name));
        }

        if !(2..=10).contains(&name.len()) {
            return Err(anyhow!(
                "名前は2文字以上10文字以下でなければなりません: {}",
                name
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
    fn test_user_name(#[case] name: &str) {
        let _ = UserName::new(name.to_string()).unwrap();
    }
}
