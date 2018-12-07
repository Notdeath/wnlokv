use std::time::Duration;

use raft::prelude::*;
use super::raftpb::Command;
use super::raftpb::CommandReply;

const METHOD_RAFTER_SEND_MSG: ::grpcio::Method<Message, Message> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/raftpb.Rafter/SendMsg",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_RAFTER_SEND_COMMAND: ::grpcio::Method<Command, CommandReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/raftpb.Rafter/SendCommand",
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

    pub fn send_command_opt(&self, req: &Command, opt: ::grpcio::CallOption) -> ::grpcio::Result<CommandReply> {
        self.client.unary_call(&METHOD_RAFTER_SEND_COMMAND, req, opt)
    }

    pub fn send_command(&self, req: &Command) -> ::grpcio::Result<CommandReply> {
        self.send_command_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_command_async_opt(&self, req: &Command, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<CommandReply>> {
        self.client.unary_call_async(&METHOD_RAFTER_SEND_COMMAND, req, opt)
    }

    pub fn send_command_async(&self, req: &Command) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<CommandReply>> {
        self.send_command_async_opt(req, ::grpcio::CallOption::call_option_default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Rafter {
    fn send_msg(&mut self, ctx: ::grpcio::RpcContext, req: Message, sink: ::grpcio::UnarySink<Message>);
    fn send_command(&mut self, ctx: ::grpcio::RpcContext, req: Command, sink: ::grpcio::UnarySink<CommandReply>);
}

pub fn create_rafter<S: Rafter + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RAFTER_SEND_MSG, move |ctx, req, resp| {
        instance.send_msg(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RAFTER_SEND_COMMAND, move |ctx, req, resp| {
        instance.send_command(ctx, req, resp)
    });
    builder.build()
}

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