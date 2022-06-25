use anyhow::{self, Context};

use domain::User;
use usecase::CreateUserCommand;

use crate::user::v1::{CreateUserRequest, CreateUserResponse, User as PbUser};

impl TryFrom<CreateUserRequest> for CreateUserCommand {
    type Error = anyhow::Error;

    fn try_from(request: CreateUserRequest) -> anyhow::Result<CreateUserCommand> {
        let CreateUserRequest { name } = request;
        let cmd = CreateUserCommand::builder()
            .name(
                name.try_into()
                    .context("failed to convert CreateUserRequest")?,
            )
            .build();
        Ok(cmd)
    }
}

impl From<User> for CreateUserResponse {
    fn from(user: User) -> CreateUserResponse {
        CreateUserResponse {
            user: Some(user.into()),
        }
    }
}

impl From<User> for PbUser {
    fn from(user: User) -> PbUser {
        PbUser {
            id: user.id().to_string(),
            name: user.name().to_string(),
        }
    }
}
