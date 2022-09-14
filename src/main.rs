use crate::pinger::pinger_server::{Pinger, PingerServer};
use crate::pinger::{PingRequest, PingResponse};
use tonic::{transport::Server, Response};

mod pinger;

#[derive(Default)]
pub struct MyPinger {}

#[tonic::async_trait]
impl Pinger for MyPinger {
    async fn ping(&self,
                  request: tonic::Request<PingRequest>
    ) -> Result<tonic::Response<PingResponse>, tonic::Status> {

        let reply = PingResponse {
            reply: format!("Hello {}", request.into_inner().msg)
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("rust-protobuf-pinger-server");

    let addr = "[::1]:5678".parse().unwrap();
    let pinger = MyPinger::default();

    println!("Listening on {}", addr);

    Server::builder()
        .add_service(PingerServer::new(pinger))
        .serve(addr)
        .await?;

    Ok(())
}
