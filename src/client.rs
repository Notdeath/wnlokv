extern crate grpcio;
extern crate protos;
extern crate tokio;

use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use futures::Future;
use tokio_core::reactor::Core;

use grpcio::{ChannelBuilder, EnvBuilder};
use protos::raftpb::HelloRequest;
use protos::raftpb_grpc::GreeterClient;

fn main() {
    let mut core = Core::new().unwrap();
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = GreeterClient::new(ch);
    let mut fvec = vec![];
    let startime = Arc::new(Instant::now());
    for i in 0..10 {
        let mut req = HelloRequest::new();
        req.set_name(format!("world:{}", i).to_owned());
        let startime = startime.clone();
        fvec.push(client.say_hello_async(&req).unwrap()
            .and_then(move |mut resp| {
                let time_cost = startime.elapsed();
                println!("{:?} ({:?})", resp, time_cost);
                Ok(())
        }).map_err(|_| ()));
        thread::sleep(Duration::from_millis(1));
        println!("One say hellow");
    }

    for f in fvec {
        tokio::run(f);
    }
    //tokio::run()
}

