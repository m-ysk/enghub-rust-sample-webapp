use tonic::transport::Server;

use app_context::AppContext;
use grpc_handler::user::v1::user_service_server::UserServiceServer;
use grpc_handler::UserServiceHandler;
use repository_impl::UserRepositoryImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url =
        std::env::var("DATABASE_URL").expect("failed to read the env var DATABASE_URL");

    let user_repository = UserRepositoryImpl::new(database_url);
    let context = AppContext { user_repository };
    let user_service = UserServiceHandler::new(context);

    let addr = "[::1]:50051".parse()?;
    println!("Start sample app server!");

    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
