use std::sync::Arc;
use std::sync::Mutex;
use tonic::{Request, Response, Status};

use super::counter_server::DecrementRequest;
use super::counter_server::DecrementResponse;
use super::counter_server::GetCurrentValueRequest;
use super::counter_server::GetCurrentValueResponse;
use super::counter_server::IncrementRequest;
use super::counter_server::IncrementResponse;
use super::counter_server::counter_server::Counter;


pub struct MyCounter {
    value: Arc<Mutex<i32>>,
}

impl MyCounter {
    pub fn new() -> Self {
        MyCounter {
            value: Arc::new(Mutex::new(0)),
        }
    }
}

#[tonic::async_trait]
impl Counter for MyCounter {
    async fn increment(&self, request: Request<IncrementRequest>) -> Result<Response<IncrementResponse>, Status> {
        let increment_value = request.into_inner().value;
        let mut value = self.value.lock().unwrap();
        *value += increment_value;

        Ok(Response::new(IncrementResponse { success: true }))
    }

    async fn decrement(&self, request: Request<DecrementRequest>) -> Result<Response<DecrementResponse>, Status> {
        let decrement_value = request.into_inner().value;
        let mut value = self.value.lock().unwrap();
        *value -= decrement_value;

        Ok(Response::new(DecrementResponse { success: true }))
    }

    async fn get_current_value(&self, _: Request<GetCurrentValueRequest>) -> Result<Response<GetCurrentValueResponse>, Status> {
        let value = self.value.lock().unwrap();

        Ok(Response::new(GetCurrentValueResponse { value: *value }))
    }
}
