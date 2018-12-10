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
use protos::commander::CommanderClient;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let mut clients: Vec<CommanderClient> = vec![];
    let mut reqcount = 1;

    let ch = ChannelBuilder::new(env.clone()).connect(&format!("localhost:{}", 48080 + reqcount * 2));
    let mut client = CommanderClient::new(ch);
    
    for i in 0..100 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);
        let mut req = Command::new();
        req.set_command_type(CommandType::CommandPut);
        req.set_key(key.as_bytes().to_vec());
        req.set_value(value.as_bytes().to_vec());
        let mut resp = CommandReply::new();
        resp = match client.send_command(&req) {
            Ok(resp) => {
                println!("[Put]:{:?} {:?}", resp, String::from_utf8(req.value.clone()));
                resp
            },
            Err(e) => {
                println!("Error is: {}", e);
                resp.set_ok(false);
                resp
            },
        };
        while resp.get_ok() != true {
            reqcount += 1;
            let ch = ChannelBuilder::new(env.clone())
                        .connect(&format!("localhost:{}", 48080 + ((reqcount % 6) + 1)* 2));
            client = CommanderClient::new(ch);
            resp = match client.send_command(&req) {
                Ok(resp) => {
                    println!("[Put]:{:?} {:?}", resp, String::from_utf8(req.value.clone()));
                    resp
                },
                Err(e) => {
                    println!("Error is: {}", e);
                    resp.set_ok(false);
                    resp
                },
            };
        }
    }

    reqcount = 1;
    println!("Put data finished");
    for i in 0..100 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);
        let mut req = Command::new();
        req.set_command_type(CommandType::CommandGet);
        req.set_key(key.as_bytes().to_vec());
        req.set_value(value.as_bytes().to_vec());
        let mut resp = CommandReply::new();
        resp = match client.send_command(&req) {
            Ok(resp) => {
                println!("[Get]:{:?}", resp);
                resp
            },
            Err(e) => {
                println!("Error is: {}", e);
                resp.set_ok(false);
                resp
            },
        };
        while resp.get_ok() != true {
            reqcount += 1;
            let ch = ChannelBuilder::new(env.clone())
                        .connect(&format!("localhost:{}", 48080 + ((reqcount % 6) + 1)* 2));
            client = CommanderClient::new(ch);
            resp = match client.send_command(&req) {
                Ok(resp) => {
                    println!("[Get]:{:?}", resp);
                    resp
                },
                Err(e) => {
                    println!("Error is: {}", e);
                    resp.set_ok(false);
                    resp
                },
            };
        }
    }
}
