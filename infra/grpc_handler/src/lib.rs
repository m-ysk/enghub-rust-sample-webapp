pub mod user {
    pub mod v1 {
        tonic::include_proto!("user.v1");
    }
}
mod convert;

use derive_getters::Getters;
use derive_new::new;
use tonic::{Request, Response, Status};

use app_context::AppContext;
use error::AppError;

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
        // gRPCのRequestをUsecaseの引数型に変換する
        let cmd = request
            .into_inner()
            .try_into()
            .map_err(|e| handle_error(e))?;

        // Usecaseを呼び出す
        let user = usecase::create_user(self.ctx(), cmd)
            .await
            .map_err(|e| handle_error(e))?;

        // Responseを返す
        Ok(Response::new(user.into()))
    }
}

fn handle_error(err: anyhow::Error) -> Status {
    // 監視のためにエラーの詳細をログ出力する。
    // バックトレースも含めて出力される。
    eprintln!("{err:?}");

    // ユーザには最も外側のエラーの内容だけを返す。
    // まずは、errの中身をAppError型にキャストできるかどうかを試す。
    match err.downcast_ref::<AppError>() {
        // AppError型の場合、種類ごとにStatusを分けてメッセージを返す
        Some(err) => match err {
            AppError::InvalidArgument(msg) => Status::invalid_argument(msg),
            AppError::Internal(msg) => Status::internal(msg),
        },
        // AppError型でない場合、ユーザに見せるべき内容かどうか分からないので、エラーが発生した旨のみ通知する
        None => Status::internal("internal error"),
    }
}
