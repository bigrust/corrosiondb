extern crate futures;
extern crate corrosiondb;
extern crate grpc;

use std::thread;

use corrosiondb::io::{
    coordinator::*,
    coordinator_grpc::*
};

struct CoordinatorImpl;

impl Coordinator for CoordinatorImpl {
    fn ping(&self, _m: grpc::RequestOptions, _req: PingRequest) -> grpc::SingleResponse<PingReply> {
        let mut reply = PingReply::new();
        reply.set_payload("pong".to_string());
        grpc::SingleResponse::completed(reply)
    }
}

fn main() {
    let port = 24322;

    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(port);
    server.add_service(CoordinatorServer::new_service_def(CoordinatorImpl));
    server.http.set_cpu_pool_threads(4);
    let _server = server.build().expect("could not build server");
    
    println!("Server started");

    loop {
        thread::park();
    }
}
