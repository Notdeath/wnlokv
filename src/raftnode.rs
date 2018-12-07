extern crate grpcio;
extern crate futures;
extern crate protobuf;

extern crate rocksdb;
use rocksdb::{DB, Writable};

use std::env;
// use futures::sync::oneshot;
use futures::Future;
use futures::future::ok;

use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink, EnvBuilder, ChannelBuilder};

use std::collections::HashMap;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::time::{Duration, Instant};
use std::sync::Arc;
use std::thread;

use raft::prelude::*;
use raft::storage::MemStorage;

use protos::raftpb::Command;
use protos::raftpb::CommandType;
use protos::raftpb::CommandReply;
use protos::raftpb_grpc::Commander;
use protos::raftpb_grpc;

use protos::rafter;

type  ProposeCallback = Box<Fn(CommandReply) + Send>;

enum Msg {
    Propose {
        id: u8,
        command: Command,
        cb: ProposeCallback,
    },
    // Here we don't use Raft Message, so use dead_code to
    // avoid the compiler warning.
    #[allow(dead_code)]
    Raft(Message),
}

struct RaftServer {
    sender: mpsc::Sender<Msg>,

}

impl RaftServer {
    fn new(sender: mpsc::Sender<Msg>, env: Arc<Environment>) ->RaftServer {
        RaftServer {
            sender,
        }
    }
}

impl Clone for RaftServer {
    fn clone(&self) ->Self {
        RaftServer {
            sender: mpsc::Sender::clone(&self.sender),
        }
    }
}

impl rafter::Rafter for RaftServer {
    fn send_msg(&mut self, ctx: RpcContext,
                req: Message, 
                sink: UnarySink<Message>) {
        // println!("Recive a req: {:?}", req);
        //let mut resp = HelloReply::new();
        let resp = Message::new();
        //let mut resp = apply_command(self.sender.clone(), req.clone());
        apply_message(mpsc::Sender::clone(&self.sender), req.clone());
        
        let f = sink.success(resp)
                .map_err(move |e| eprintln!("Fail to reply {:?}: {:?}", req, e));
        ctx.spawn(f);
    }

    fn send_command(&mut self, ctx: RpcContext,
                req: Command, 
                sink: UnarySink<CommandReply>) {
        println!("Recive a req: {:?}", req);
        //let mut resp = HelloReply::new();
        let resp = apply_command(mpsc::Sender::clone(&self.sender), req.clone());
        
        let f = sink.success(resp)
                .map_err(move |e| eprintln!("Fail to reply {:?}: {:?}", req, e));
        ctx.spawn(f);
    }
}


struct CommandServer {
    sender: mpsc::Sender<Msg>,
}

impl Clone for CommandServer {
    fn clone(&self) ->Self {
        CommandServer {
            sender:  mpsc::Sender::clone(&self.sender),
        }
    }
}

impl CommandServer {
    fn new(sender: mpsc::Sender<Msg>, env: Arc<Environment>) ->CommandServer {
        CommandServer {
            sender,
        }
    }
}


impl Commander for CommandServer {
    fn send_command(&mut self, ctx: RpcContext,
                req: Command, 
                sink: UnarySink<CommandReply>) {
        println!("Recive a req: {:?}", req);
        //let mut resp = HelloReply::new();
        let f = ok::<u32, u32>(1);
        let sender = mpsc::Sender::clone(&self.sender);
        let f = f.then(move |_| {
            let resp = apply_command(sender, req);
            Ok(resp)
        })
        .and_then(|res| sink.success(res)
                .map_err(move |e| eprintln!("Fail to reply: {:?}", e)))
        .map(|_| ())
        .map_err(|_| ());
        ctx.spawn(f);
    }
}

static mut seq: u8 = 1;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Try use cargo run 5 1");
        return ;
    }
    let mut arg = &args[1];
    let num: u64 = arg.trim().parse().unwrap();
    arg = &args[2];
    let port_num: u64 = arg.trim().parse().unwrap();
    let port = 48080 + 2 * port_num;

    let rocks_db: DB = DB::open_default(&format!("/path/for/rocksdb/storage{}", port_num)).unwrap();
    let storage = MemStorage::new();
    let cfg = Config{
        id: port_num,
        peers: vec![],
        election_tick: 10,
        heartbeat_tick: 3,
        max_size_per_msg: 1024 * 1024 * 1024,
        max_inflight_msgs: 256,
        applied: 0,
        tag: format!("[{}]", port_num),
        ..Default::default()
    };
    let mut peers = vec![];
    let peer_count = num+1;
    for i in 1..peer_count {
        let peer_ip = format!("127.0.0.1:{}", 48080 + 2 * i + 1) ;
        peers.push(Peer {
            id: i as u64,
            context: Some(peer_ip.as_bytes().to_vec()),
        });
    }
    let env = Arc::new(EnvBuilder::new().build());
    let mut r = RawNode::new(&cfg, storage, peers).unwrap();
    let (sender, receiver) = mpsc::channel();

    let service = raftpb_grpc::create_commander(CommandServer {
        sender: mpsc::Sender::clone(&sender),
    });
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind("127.0.0.1", port as u16)
        .build().unwrap();
    
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("Listening on {}:{}", host, port);
    }
    let env = Arc::new(EnvBuilder::new().build());
    let mut raft_server = RaftServer::new(mpsc::Sender::clone(&sender), env.clone());

    let service = rafter::create_rafter(raft_server.clone());
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind("127.0.0.1", port as u16 + 1)
        .build().unwrap();
    
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("Listening on {}:{}", host, port);
    }

    // send_propose(sender);
    let mut t = Instant::now();
    let mut timeout = Duration::from_millis(100);
    let mut cbs = HashMap::new();
    let mut peer_clients = HashMap::new();
    let (ms, mr)  = mpsc::channel();
    msg_send_thread(mr, env.clone(), port_num);

    loop {
        match receiver.recv_timeout(timeout) {
            Ok(Msg::Propose { id, command, cb }) => {
                let is_leader = r.raft.leader_id == r.raft.id;
                if is_leader {
                    println!("is leader");
                    // if command.get_command_type() == CommandType::CommandPut {
                    //     let mut reply = CommandReply::new();
                    //     reply.set_ok(true);
                    //     //reply.set_value();
                    //     cb(reply);
                    // } else if command.get_command_type() == CommandType::CommandGet {
                    //     let reply = use_commnad(&rocks_db, &command);
                    //     //reply.set_ok(true);
                    //     //reply.set_value();
                    //     cb(reply);
                    // }
                    cbs.insert(id,cb);
                    let data = protobuf::Message::write_to_bytes(&command).unwrap();
                    println!("Call Data is: {:?}", data);
                    r.propose(vec![id], data).unwrap();
                } else {
                    println!("is not leader");
                    let mut reply = CommandReply::new();
                    reply.set_ok(false);
                    //reply.set_value();
                    cb(reply);
                }
            },
            Ok(Msg::Raft(m)) => {
                // println!("recived msg is: {:?}", m);
                match m.msg_type {
                    MessageType::MsgAppend |  MessageType::MsgAppendResponse=> {
                        println!("recived msg is: {:?}", m);
                    },
                    _=> {},

                }
                r.step(m).unwrap();
            },
            Err(RecvTimeoutError::Timeout) => (),
            Err(RecvTimeoutError::Disconnected) => return,
        }

        let d = t.elapsed();
        if d >= timeout {
            t = Instant::now();
            timeout = Duration::from_millis(100);
            // We drive Raft every 100ms.
            // println!("raft tick: State is {:?}", r.raft.state);
            r.tick();
        } else {
            timeout -= d;
        }
        let ms1 = mpsc::Sender::clone(&ms);
        on_ready(&mut r, &mut cbs, &mut peer_clients, ms1, env.clone(), &rocks_db);
    }
    
}

fn on_ready(r: &mut RawNode<MemStorage>,
            cbs: &mut HashMap<u8, ProposeCallback>, 
            peer_clients: &mut HashMap<u64, Box<rafter::RafterClient>>,
            sender: mpsc::Sender<Message>, 
            env: Arc<Environment>, 
            rocks_db: &DB) {
    if !r.has_ready() {
        return;
    }

    // The Raft is ready, we can do something now.
    let mut ready = r.ready();

    let is_leader = r.raft.leader_id == r.raft.id;
    if is_leader {
        // If the peer is leader, the leader can send messages to other followers ASAP.
        let msgs = ready.messages.drain(..);
        println!("ready messages is {:?}", msgs);
        // let mut fvec = vec![];
        for msg in msgs {
            sender.send(msg).unwrap();
            // let to = msg.get_to();
            // if to != r.raft.id {
            //     let ch = ChannelBuilder::new(env.clone())
            //         .connect(&format!("127.0.0.1:{}", 48080 + to * 2 + 1));
            //     let client = rafter::RafterClient::new(ch);
            //     fvec.push(client.send_msg_async(&msg).unwrap()
            //         .and_then(move |resp| {
            //             Ok(())
            //         }).map_err(|_| ()));
            // }
            // Here we only have one peer, so can ignore this.
        }
        // for f in fvec {
        //     tokio::run(f);
        // }
    }

    if !raft::is_empty_snap(ready.snapshot()) {
        // This is a snapshot, we need to apply the snapshot at first.
        r.mut_store()
            .wl()
            .apply_snapshot(ready.snapshot().clone())
            .unwrap();
    }

    if !ready.entries().is_empty() {
        // Append entries to the Raft log
        r.mut_store().wl().append(ready.entries()).unwrap();
    }

    if let Some(hs) = ready.hs() {
        // Raft HardState changed, and we need to persist it.
        println!("Raft HardState changed");
        r.mut_store().wl().set_hardstate(hs.clone());
    }

    if !is_leader {
        // If not leader, the follower needs to reply the messages to
        // the leader after appending Raft entries.
        let msgs = ready.messages.drain(..);
        // println!("ready messages is {:?}", msgs);
        // let mut fvec = vec![];
        for msg in msgs {
            match msg.msg_type {
                MessageType::MsgAppend |  MessageType::MsgAppendResponse=> {
                    println!("To sennd msg is: {:?}", msg);
                },
                _=> {},
            }
            sender.send(msg).unwrap();
            //println!("send msg ok");
            // let to = msg.get_to();
            // if to != r.raft.id {
            //     let ch = ChannelBuilder::new(env.clone())
            //         .connect(&format!("127.0.0.1:{}", 48080 + to * 2 + 1));
            //     let client = rafter::RafterClient::new(ch);
            //     fvec.push(client.send_msg_async(&msg).unwrap()
            //         .and_then(move |resp| {
            //             println!("Send ok {:?}", resp);
            //             Ok(())
            //         }).map_err(|_| ()));
            // }
            // Here we only have one peer, so can ignore this.
        }
        // for f in fvec {
        //     tokio::run(f);
        // }
    }

    if let Some(committed_entries) = ready.committed_entries.take() {
        let mut _last_apply_index = 0;
        for entry in committed_entries {
            // Mostly, you need to save the last apply index to resume applying
            // after restart. Here we just ignore this because we use a Memory storage.
            _last_apply_index = entry.get_index();

            if entry.get_data().is_empty() {
                // Emtpy entry, when the peer becomes Leader it will send an empty entry.
                continue;
            }

            if entry.get_entry_type() == EntryType::EntryNormal {
                println!("Data is: {}", String::from_utf8(entry.get_data().to_vec()).unwrap());
                use protobuf::Message;
                let mut command = Command::new();
                command.merge_from_bytes(entry.get_data()).unwrap();
                let reply = use_commnad(rocks_db, &command);
                if let Some(cb) = cbs.remove(entry.get_context().get(0).unwrap()) {
                    println!("start call back");
                    cb(reply);
                } else {
                    println!("Wrong call back");
                }
            } else {

                use protobuf::Message;
                println!("Entry is: {:?}", entry);
                let mut cc = ConfChange::new();
                cc.merge_from_bytes(&entry.get_data()).unwrap();
                println!("ConfChange: {:?}", cc);
                println!("Append ip is {}", String::from_utf8(cc.get_context().to_vec()).unwrap());
                match cc.get_change_type() {
                    ConfChangeType::RemoveNode => {
                        peer_clients.remove(&cc.get_node_id());
                    },
                    _ => {
                        if cc.get_node_id() != r.raft.id {
                            // let ch = ChannelBuilder::new(env.clone()).connect(
                            //         &String::from_utf8(cc.get_context().to_vec()).unwrap());
                            // let client = rafter::RafterClient::new(ch);
                            // peer_clients.insert(cc.get_node_id(), Box::new(client));
                        }
                    }
                }
                // r.apply_conf_change(&cc);
            }

            // TODO: handle EntryConfChange
        }
    }

    // Advance the Raft
    r.advance(ready);
}

fn msg_send_thread(receiver: mpsc::Receiver<Message>, env: Arc<Environment>, myid: u64) {
    // let mut peer_clients = HashMap::new();
    for i in 1..6 {
        if i == myid {
            continue;
        }
        // let ch = ChannelBuilder::new(env.clone())
        //             .connect(&format!("127.0.0.1:{}", 48080 + i * 2 + 1));
        // let client = rafter::RafterClient::new(ch);
        // peer_clients.insert(i as u64, Box::new(client));
    }

    thread::spawn(move || {loop {
        let msg = receiver.recv().unwrap();
        //println!("[Debug]({}, {}), recived {:?}", file!(), line!(), msg);
        let to = msg.get_to();
        if to != myid {
            let env = env.clone();
            thread::spawn(move || {
            let ch = ChannelBuilder::new(env)
                    .connect(&format!("127.0.0.1:{}", 48080 + to * 2 + 1));
            let client = rafter::RafterClient::new(ch);
            match client.send_msg(&msg) {
                Ok(resp) => {
                    println!("[Debug]({}, {}), Send ok  {:?}", file!(), line!(), resp);
                },
                Err(e) =>{
                    println!("Send error is: {}", e);
                }
            }});
        }
    }});
}

fn use_commnad(rocks_db: &DB, command: &Command) -> CommandReply{
    let mut command_reply = CommandReply::new();
    match command.get_command_type() {
        CommandType::CommandPut => {
            match rocks_db.put(&command.key[..], &command.value[..]) {
                Ok(_) =>{ 
                    command_reply.set_ok(true);
                },
                Err(e) =>{ 
                    command_reply.set_ok(false);
                    command_reply.set_value(e.as_bytes().to_vec());
                },
            }
        },
        CommandType::CommandDelete => {
            match rocks_db.delete(&command.key[..]) {
                Ok(_) =>{ 
                    command_reply.set_ok(true);
                },
                Err(e) =>{ 
                    command_reply.set_ok(false);
                    command_reply.set_value(e.as_bytes().to_vec());
                },
            }
        },
        CommandType::CommandGet => {
            match rocks_db.get(&command.key[..]) {
                Ok(Some(value)) =>{ 
                    command_reply.set_ok(true);
                    command_reply.set_value(value.to_vec());
                },
                Ok(None) => {
                    command_reply.set_ok(true);
                    command_reply.set_value(b"No value to get".to_vec());
                },
                Err(e) => { 
                    command_reply.set_ok(false);
                    command_reply.set_value(e.as_bytes().to_vec());
                },
            }
        },
    }
    println!("Command reply value is {:?}", command_reply.get_value());
    command_reply
}


fn apply_command(sender: mpsc::Sender<Msg>, command: Command) 
    ->CommandReply {
    let (s1, r1) = mpsc::channel::<CommandReply>();

    println!("Propose a command");
    unsafe{
    seq = seq  % 127 + 1;
    
    sender.send(Msg::Propose {
        id: seq,
        command,
        cb: Box::new(move |reply| {
            s1.send(reply).unwrap();
        }),
    }).unwrap();
    }
    let reply = r1.recv().unwrap();

    println!("Receive the commnad reply");
    reply
}

fn apply_message(sender: mpsc::Sender<Msg>, message: Message) {
    // println!("Propose a Message");

    // println!("Message is: {:?}", message);

    sender.send(Msg::Raft(message)).unwrap();

    // println!("Receive the commnad reply");
}
