use auth_service::{auth::auth_server::AuthServer, auth_impl::AuthService};
use counter::{counter::counter_server::CounterServer, counter_impl::MyCounter};
use echo_service::{echo::echo_server::EchoServer, echo_impl::EchoService};
use std::env;
use tonic::{metadata::MetadataValue, transport::Server, Request, Status};
use tonic_reflection::server::Builder;

mod auth_service;
mod counter;
mod echo_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let addr = env::var("GRPC_SERVER_ADDRESS")?.parse().unwrap();
    let counter = MyCounter::new();
    let auth_service = AuthService::new(b"secret".to_vec());
    let echo_service = EchoService::default();
    let svc = EchoServer::with_interceptor(echo_service, check_auth);

    let reflection_path = "pb/reflection_descriptor.bin";
    let file = std::fs::read(reflection_path)?;

    Server::builder()
        .add_service(CounterServer::new(counter))
        .add_service(AuthServer::new(auth_service))
        .add_service(svc)
        .add_service(
            Builder::configure()
                .register_encoded_file_descriptor_set(&file)
                .build()
                .unwrap(),
        )
        // todo add tonic reflection
        .serve(addr)
        .await?;

    Ok(())
}

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token: MetadataValue<_> = "Bearer some-secret-token".parse().unwrap();

    match req.metadata().get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}
