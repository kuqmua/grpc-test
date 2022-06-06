use tonic::{transport::Server, Request, Response, Status};

use payments::Bitcoin_server::{Bitcoin, BitcoinServer};
use payments::{BtcPaymentResponse, BtcPaymentRequest};

pub mod payments {
    tonic::include_proto!("payments");
}

fn main() {
    println!("Hello, world!");
}
