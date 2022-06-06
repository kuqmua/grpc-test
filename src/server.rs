use tonic::{transport::Server, Request, Response, Status};

use payments::Bitcoin_server::{Bitcoin, BitcoinServer};
use payments::{BtcPaymentResponse, BtcPaymentRequest};

pub mod payments {
    tonic::include_proto!("payments");
}
#[derive(Debug, Default)]
pub struct BitcoinService {}

#[tonic::async_trait]
impl Bitcoin for BitcoinService{
    async fn send_payment(
        &self, 
        request: Request<BtcPaymentRequest>,
    ) -> Result<Response<BtcPaymentResponse>, Status>{
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = BtcPaymentResponse {
            successful: true,
            message: format!("Sent {}BTC to {}.", req.amount, req.to_addr).into(),
        };
        Ok(Response::new(reply))
    }
}

fn main() {
    println!("Hello, world!");
}
