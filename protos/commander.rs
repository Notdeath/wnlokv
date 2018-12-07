const METHOD_COMMANDER_SEND_COMMAND: ::grpcio::Method<super::raftpb::Command, super::raftpb::CommandReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/commander.Commander/SendCommand",
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