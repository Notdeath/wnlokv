// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_GREETER_SAY_HELLO: ::grpcio::Method<super::raftpb::HelloRequest, super::raftpb::HelloReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/raftpb.Greeter/SayHello",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct GreeterClient {
    client: ::grpcio::Client,
}

impl GreeterClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        GreeterClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn say_hello_opt(&self, req: &super::raftpb::HelloRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::raftpb::HelloReply> {
        self.client.unary_call(&METHOD_GREETER_SAY_HELLO, req, opt)
    }

    pub fn say_hello(&self, req: &super::raftpb::HelloRequest) -> ::grpcio::Result<super::raftpb::HelloReply> {
        self.say_hello_opt(req, ::grpcio::CallOption::default())
    }

    pub fn say_hello_async_opt(&self, req: &super::raftpb::HelloRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raftpb::HelloReply>> {
        self.client.unary_call_async(&METHOD_GREETER_SAY_HELLO, req, opt)
    }

    pub fn say_hello_async(&self, req: &super::raftpb::HelloRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raftpb::HelloReply>> {
        self.say_hello_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Greeter {
    fn say_hello(&mut self, ctx: ::grpcio::RpcContext, req: super::raftpb::HelloRequest, sink: ::grpcio::UnarySink<super::raftpb::HelloReply>);
}

pub fn create_greeter<S: Greeter + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GREETER_SAY_HELLO, move |ctx, req, resp| {
        instance.say_hello(ctx, req, resp)
    });
    builder.build()
}

// const METHOD_RAFTER_SEND_MSG: ::grpcio::Method<super::raftpb::Message, super::raftpb::Message> = ::grpcio::Method {
//     ty: ::grpcio::MethodType::Unary,
//     name: "/raftpb.Rafter/SendMsg",
//     req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
//     resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
// };

// #[derive(Clone)]
// pub struct RafterClient {
//     client: ::grpcio::Client,
// }

// impl RafterClient {
//     pub fn new(channel: ::grpcio::Channel) -> Self {
//         RafterClient {
//             client: ::grpcio::Client::new(channel),
//         }
//     }

//     pub fn send_msg_opt(&self, req: &super::raftpb::Message, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::raftpb::Message> {
//         self.client.unary_call(&METHOD_RAFTER_SEND_MSG, req, opt)
//     }

//     pub fn send_msg(&self, req: &super::raftpb::Message) -> ::grpcio::Result<super::raftpb::Message> {
//         self.send_msg_opt(req, ::grpcio::CallOption::default())
//     }

//     pub fn send_msg_async_opt(&self, req: &super::raftpb::Message, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raftpb::Message>> {
//         self.client.unary_call_async(&METHOD_RAFTER_SEND_MSG, req, opt)
//     }

//     pub fn send_msg_async(&self, req: &super::raftpb::Message) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raftpb::Message>> {
//         self.send_msg_async_opt(req, ::grpcio::CallOption::default())
//     }
//     pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
//         self.client.spawn(f)
//     }
// }

// pub trait Rafter {
//     fn send_msg(&mut self, ctx: ::grpcio::RpcContext, req: super::raftpb::Message, sink: ::grpcio::UnarySink<super::raftpb::Message>);
// }

// pub fn create_rafter<S: Rafter + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
//     let mut builder = ::grpcio::ServiceBuilder::new();
//     let mut instance = s.clone();
//     builder = builder.add_unary_handler(&METHOD_RAFTER_SEND_MSG, move |ctx, req, resp| {
//         instance.send_msg(ctx, req, resp)
//     });
//     builder.build()
// }

const METHOD_COMMANDER_SEND_COMMAND: ::grpcio::Method<super::raftpb::Command, super::raftpb::CommandReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/raftpb.Commander/SendCommand",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct CommanderClient {
    client: ::grpcio::Client,
}

impl CommanderClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        CommanderClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn send_command_opt(&self, req: &super::raftpb::Command, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::raftpb::CommandReply> {
        self.client.unary_call(&METHOD_COMMANDER_SEND_COMMAND, req, opt)
    }

    pub fn send_command(&self, req: &super::raftpb::Command) -> ::grpcio::Result<super::raftpb::CommandReply> {
        self.send_command_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_command_async_opt(&self, req: &super::raftpb::Command, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raftpb::CommandReply>> {
        self.client.unary_call_async(&METHOD_COMMANDER_SEND_COMMAND, req, opt)
    }

    pub fn send_command_async(&self, req: &super::raftpb::Command) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raftpb::CommandReply>> {
        self.send_command_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Commander {
    fn send_command(&mut self, ctx: ::grpcio::RpcContext, req: super::raftpb::Command, sink: ::grpcio::UnarySink<super::raftpb::CommandReply>);
}

pub fn create_commander<S: Commander + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_COMMANDER_SEND_COMMAND, move |ctx, req, resp| {
        instance.send_command(ctx, req, resp)
    });
    builder.build()
}
