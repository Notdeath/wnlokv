extern crate grpcio;
extern crate protos;
extern crate tokio;
//extern crate rocksdb;
//varuse rocksdb::{DB, Writable};

use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use futures::Future;
use tokio_core::reactor::Core;

use grpcio::{ChannelBuilder, EnvBuilder};
use protos::raftpb::{CommandType, Command};
use protos::raftpb_grpc::CommanderClient;

fn main() {
    let mut core = Core::new().unwrap();
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:8080");
    let client = CommanderClient::new(ch);
    let mut fvec = vec![];
    let startime = Arc::new(Instant::now()); 
    for i in 0..10 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);
        let mut req = Command::new();
        req.set_command_type(CommandType::CommandPut);
        req.set_key(key.as_bytes().to_vec());
        req.set_value(value.as_bytes().to_vec());
        let startime = startime.clone();
        fvec.push(client.send_command_async(&req).unwrap()
            .and_then(move |mut resp| {
                let time_cost = startime.elapsed();
                println!("{:?} ({:?})", resp, time_cost);
                println!("value is {:?}", resp.get_value());
                Ok(())
        }).map_err(|_| ()));
        thread::sleep(Duration::from_millis(1));
        println!("One say hello");
    }

    for f in fvec {
        tokio::run(f);
    }
    
    let mut fvec = vec![];
    let startime = Arc::new(Instant::now()); 
    for i in 0..10 {
        let key = format!("key{}", i);
        let mut req = Command::new();
        req.set_command_type(CommandType::CommandGet);
        req.set_key(key.as_bytes().to_vec());
        let startime = startime.clone();
        fvec.push(client.send_command_async(&req).unwrap()
            .and_then(move |mut resp| {
                let time_cost = startime.elapsed();
                println!("{:?} ({:?})", resp, time_cost);
                println!("value is {:?}", String::from_utf8(resp.value));
                Ok(())
        }).map_err(|_| ()));
        thread::sleep(Duration::from_millis(1));
        println!("One say hello");
    }

    for f in fvec {
        tokio::run(f);
    }

        //tokio::run()
}

