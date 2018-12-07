extern crate futures;
extern crate grpcio;
extern crate protos;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};
use std::time::Duration;

use futures::sync::oneshot;
use futures::Future;
use futures::future::ok;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use protos::raftpb::HelloRequest;
use protos::raftpb::HelloReply;
use protos::raftpb_grpc;
use protos::raftpb_grpc::Greeter;

#[derive(Clone)]
struct GreeterService;

impl Greeter for GreeterService {
    fn say_hello(&mut self, ctx: RpcContext, req: HelloRequest,
                 sink: UnarySink<HelloReply>) {
 
        println!("Recive a req: {:?}", req);
        let msg = format!("Hello {}", req.get_name());
        let f = ok::<u32, u32>(1);
        let f = f.then(move |_| {
            println!("Req is: {:?}", req);
            let msg = format!("Hello {}", req.get_name());
            thread::sleep(Duration::from_secs(1));
            let mut resp = HelloReply::new();
            resp.set_message(msg);
            Ok(resp)
        })
        .and_then(|res| sink.success(res)
                .map_err(move |e| eprintln!("Fail to reply: {:?}", e)))
        .map(|_| ())
        .map_err(|_| ());
        ctx.spawn(f);
    }
}



fn main() {
    let env = Arc::new(Environment::new(20));
    let service = raftpb_grpc::create_greeter(GreeterService);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50051)
        .build().unwrap();

    server.start();

    for &(ref host, port) in server.bind_addrs() {
        println!("Listening on {}:{}", host, port);
    }

    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press Enter to exit ...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(()).unwrap();
    });

    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
