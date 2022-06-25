pub mod user {
    pub mod v1 {
        tonic::include_proto!("user.v1");
    }
}

use derive_getters::Getters;
use derive_new::new;
use tonic::{Request, Response, Status};

use app_context::AppContext;
use usecase::CreateUserCommand;

use user::v1::user_service_server::UserService;
use user::v1::{CreateUserRequest, CreateUserResponse};

#[derive(new, Getters)]
pub struct UserServiceHandler {
    ctx: AppContext,
}

#[tonic::async_trait]
impl UserService for UserServiceHandler {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        println!("Got a request: {request:?}");

        let response = usecase::create_user(
            self.ctx(),
            CreateUserCommand::builder()
                .name("test".to_string().try_into().unwrap())
                .build(),
        )
        .await
        .unwrap();

        println!("Response: {response:?}");

        Err(Status::unimplemented("unimplemented!"))
    }
}
