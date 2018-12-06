extern crate grpcio;
extern crate protos;
extern crate tokio;
//extern crate rocksdb;
//varuse rocksdb::{DB, Writable};

use std::sync::Arc;
//use std::thread;
//use std::time::{Duration, Instant};
//use futures::Future;

use grpcio::{ChannelBuilder, EnvBuilder};
use protos::raftpb::{CommandType, Command, CommandReply};
use protos::rafter::RafterClient;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let mut clients: Vec<RafterClient> = vec![];
    for i in 1.. 6 {
        let ch = ChannelBuilder::new(env.clone()).connect(&format!("localhost:{}", 8080 + i * 2));
        let client = RafterClient::new(ch);
        clients.push(
           client 
        );
    }

    for i in 0..100 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);
        let mut req = Command::new();
        req.set_command_type(CommandType::CommandPut);
        req.set_key(key.as_bytes().to_vec());
        req.set_value(value.as_bytes().to_vec());
        for client in clients.iter() {
            println!("begin send");
            let resp = match client.send_command(&req) {
                Ok(resp) => resp,
                Err(e) => {
                    println!("Error is: {}", e);
                    CommandReply::new()
                },
            };
            println!("Out send");
            if resp.ok == true {
                println!("Put ok: {:?}", resp);
                break;
            }
        }
    }

    for i in 0..100 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);
        let mut req = Command::new();
        req.set_command_type(CommandType::CommandGet);
        req.set_key(key.as_bytes().to_vec());
        req.set_value(value.as_bytes().to_vec());
        for client in clients.iter() {
            let resp = match client.send_command(&req) {
                Ok(resp) => resp,
                Err(e) => {
                    println!("Error is: {}", e);
                    CommandReply::new()
                },
            };
            if resp.ok == true {
                println!("value is {:?}", String::from_utf8(resp.value));
                break;
            }
        }
    }
        //tokio::run()
}

