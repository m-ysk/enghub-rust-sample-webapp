pub mod user {
    pub mod v1 {
        tonic::include_proto!("user.v1");
    }
}

use tonic::{Request, Response, Status};

use user::v1::user_service_server::UserService;
use user::v1::{CreateUserRequest, CreateUserResponse};

#[derive(Debug, Default)]
pub struct UserServiceHandler {}

#[tonic::async_trait]
impl UserService for UserServiceHandler {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        println!("Got a request: {request:?}");

        Err(Status::unimplemented("unimplemented!"))
    }
}
