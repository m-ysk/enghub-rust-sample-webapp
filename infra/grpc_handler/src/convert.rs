use anyhow::{self, Context};

use domain::{User, UserId};
use error::AppError;
use usecase::CreateUserCommand;

use crate::user::v1::{
    CreateUserRequest, CreateUserResponse, GetUsersByIdsRequest, GetUsersByIdsResponse,
    User as PbUser,
};

impl TryFrom<CreateUserRequest> for CreateUserCommand {
    type Error = anyhow::Error;

    fn try_from(request: CreateUserRequest) -> anyhow::Result<CreateUserCommand> {
        let CreateUserRequest { name } = request;
        let cmd = CreateUserCommand::builder()
            .name(
                name.try_into()
                    .with_context(|| AppError::InvalidArgument(format!("invalid name")))?,
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

impl TryFrom<GetUsersByIdsRequest> for Vec<UserId> {
    type Error = anyhow::Error;

    fn try_from(request: GetUsersByIdsRequest) -> anyhow::Result<Vec<UserId>> {
        let GetUsersByIdsRequest { ids } = request;
        ids.into_iter().map(|id| id.parse()).collect()
    }
}

impl From<Vec<User>> for GetUsersByIdsResponse {
    fn from(users: Vec<User>) -> GetUsersByIdsResponse {
        let users = users.into_iter().map(From::from).collect();
        GetUsersByIdsResponse { users }
    }
}
