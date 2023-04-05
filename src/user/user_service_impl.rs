use tonic::{Request, Response, Status};

use super::user_service_server::{user_service_server::UserService, RegisterRequest, RegisterResponse, LoginRequest, LoginResponse};

pub struct MyUserService {}

#[tonic::async_trait]
impl UserService for MyUserService {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        // Implement your business logic here
        // get email , name and username
        let email = request.get_ref().email.clone();
        let name = request.get_ref().name.clone();
        let username = request.get_ref().username.clone();

        let message = format!("Register successful for {} {} {}", email, name, username);

        let response = RegisterResponse {
            message: message.to_string(),
        };
        Ok(Response::new(response))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        // Implement your business logic here
        // get username
        let username = request.get_ref().username.clone();
        let message  = format!("Login successful for {}", username);

        let response = LoginResponse {
            message: message.to_string(),
        };
        Ok(Response::new(response))
    }
}
