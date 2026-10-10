use service::service_server::{Service, ServiceServer};
use tonic::{Request, Response, Status, transport::Server};

use crate::service::{GetInfoRequest, GetInfoResponse};

#[derive(Default)]
pub struct ServiceImpl {}

#[tonic::async_trait]
impl Service for ServiceImpl {
    async fn get_info(
        &self,
        _request: Request<GetInfoRequest>,
    ) -> Result<Response<GetInfoResponse>, Status> {
        println!("Got a request");
        let reply = GetInfoResponse {
            info: "Mock Service on Tonic: 0.14.6".to_string(),
        };
        Ok(Response::new(reply))
    }
}

pub mod service {
    tonic::include_proto!("service");
}

#[tokio::main]
async fn main() {
    let service = ServiceImpl::default();
    let addr = "0.0.0.0:50051".parse().unwrap();
    println!("gRPC listening on :50051");
    Server::builder()
        .add_service(ServiceServer::new(service))
        .serve(addr)
        .await
        .unwrap();
}
