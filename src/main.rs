use auth_service::{ auth::auth_server::AuthServer, auth_impl::AuthService};
use counter::{counter_impl::MyCounter, counter::counter_server::CounterServer};
use tonic::transport::Server;
use std::env;

mod counter;
mod auth_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let addr = env::var("GRPC_SERVER_ADDRESS")?.parse().unwrap();
    let counter = MyCounter::new();
    let auth_service = AuthService::new(b"secret".to_vec());

    Server::builder()
        .add_service(CounterServer::new(counter))
        .add_service(AuthServer::new(auth_service))
        // todo add tonic reflection
        .serve(addr)
        .await?;

    Ok(())
}
