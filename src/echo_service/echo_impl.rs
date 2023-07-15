use tonic::{Response, Status, Request};

use super::echo::{echo_server::Echo, EchoRequest, EchoResponse};

type EchoResult<T> = Result<Response<T>, Status>;

#[derive(Default)]
pub struct EchoService;

#[tonic::async_trait]
impl Echo for EchoService {
    async fn unary_echo(&self, request: Request<EchoRequest>) -> EchoResult<EchoResponse> {
        let message = request.into_inner().message;
        Ok(Response::new(EchoResponse { message }))
    }
}
