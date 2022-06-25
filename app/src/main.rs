use tonic::transport::Server;

use grpc_handler::user::v1::user_service_server::UserServiceServer;
use grpc_handler::UserServiceHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let user_service = UserServiceHandler::default();

    println!("Start sample app server!");

    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
