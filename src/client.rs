use payments::Bitcoin_client::BitcoinClient;
use payments::BtcPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client =  BitcoinClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(
        BtcPaymentRequest {
            from_addr: "123456".to_owned(),
            to_addr: "654321".to_owned(),
            amount: 22,
        }
    );
    Ok(())
}