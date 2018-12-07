use std::time::Duration;
use raft::prelude::*;

const METHOD_RAFTER_SEND_MSG: ::grpcio::Method<Message, Message> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/raftpb.Rafter/SendMsg",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct RafterClient {
    client: ::grpcio::Client,
}

trait MyDefault {
    fn call_option_default() -> ::grpcio::CallOption;
}



impl MyDefault for ::grpcio::CallOption {
    fn call_option_default() ->grpcio::CallOption {
        let op = ::grpcio::CallOption::default();
        op.timeout(Duration::from_millis(100))
    }
}

impl RafterClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        RafterClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn send_msg_opt(&self, req: &Message, opt: ::grpcio::CallOption) -> ::grpcio::Result<Message> {
        self.client.unary_call(&METHOD_RAFTER_SEND_MSG, req, opt)
    }

    pub fn send_msg(&self, req: &Message) -> ::grpcio::Result<Message> {
        self.send_msg_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_msg_async_opt(&self, req: &Message, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<Message>> {
        self.client.unary_call_async(&METHOD_RAFTER_SEND_MSG, req, opt)
    }

    pub fn send_msg_async(&self, req: &Message) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<Message>> {
        self.send_msg_async_opt(req, ::grpcio::CallOption::call_option_default())
    }

    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Rafter {
    fn send_msg(&mut self, ctx: ::grpcio::RpcContext, req: Message, sink: ::grpcio::UnarySink<Message>);
}

pub fn create_rafter<S: Rafter + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RAFTER_SEND_MSG, move |ctx, req, resp| {
        instance.send_msg(ctx, req, resp)
    });
    builder.build()
}