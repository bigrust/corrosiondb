extern crate corrosiondb;
extern crate grpc;
extern crate futures;

use grpc::Client;

use corrosiondb::io::{
    coordinator::*,
    coordinator_grpc::*
};

fn main() {
    let grpc_client = Client::new_plain("::1", 24322, Default::default()).unwrap();
    let client = CoordinatorClient::with_client(grpc_client);

    let mut req = PingRequest::new();
    req.set_payload("ping".to_string());

    let resp = client.ping(grpc::RequestOptions::new(), req);
    let ping_reply = resp.wait().unwrap().1;
    println!("{:?}", ping_reply);
}