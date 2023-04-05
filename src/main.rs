use tonic::transport::Server;

use hello_world::{hello_world_impl::MyGreeter, hello_world_server::greeter_server::GreeterServer};

use counter::{counter_impl::MyCounter, counter_server::counter_server::CounterServer};
use user::{user_service_impl::MyUserService, user_service_server::user_service_server::UserServiceServer};

mod counter;
mod hello_world;
mod user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let greeter = MyGreeter {};
    let counter = MyCounter::new();
    let auth_service = MyUserService {};

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(CounterServer::new(counter))
        .add_service(UserServiceServer::new(auth_service))
        .serve(addr)
        .await?;

    Ok(())
}
