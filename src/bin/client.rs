extern crate corrosiondb;
extern crate grpc;
extern crate futures;
extern crate rustyline;

use grpc::Client;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use corrosiondb::io::{
    coordinator::*,
    coordinator_grpc::*
};

fn main() {
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let cmd = parse_command(line).unwrap();
                let result = handle_command(cmd);

                match result {
                    Ok(ok) => {
                        println!("{}", ok)
                    },
                    Err(err) => {
                        println!("ERROR:{}", err)
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}

enum Command {
    Ping,
}

fn parse_command(_command: String) -> Result<Command, ()> {
    Ok(Command::Ping)
}

fn handle_command(cmd: Command) -> Result<String, String> {
    match cmd {
        Command::Ping => {
            let grpc_client = Client::new_plain("::1", 24322, Default::default()).unwrap();
            let client = CoordinatorClient::with_client(grpc_client);

            let mut req = PingRequest::new();
            req.set_payload("ping".to_string());

            let resp = client.ping(grpc::RequestOptions::new(), req);
            let ping_reply = resp.wait().map_err(|_| "grpc error".to_string()) ?.1;

            Ok(ping_reply.get_payload().to_string())
        }
    }
}