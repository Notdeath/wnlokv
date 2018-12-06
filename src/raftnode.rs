#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate grpcio;
extern crate futures;
extern crate protobuf;

extern crate rocksdb;
use rocksdb::{DB, Writable};

use std::env;
// use futures::sync::oneshot;
use futures::Future;

use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink, EnvBuilder, ChannelBuilder};

use std::collections::HashMap;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};
use std::sync::Arc;

use raft::prelude::*;
use raft::storage::MemStorage;

use protos::raftpb::Command;
// use protos::raftpb::CommandType;
use protos::raftpb::CommandReply;

use protos::rafter;

use protobuf::ProtobufEnum;

use protos::raftpb_grpc::{self, *};

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

#[derive(Serialize, Deserialize, Debug)]
struct RaftCommand {
    op: i32,
    key: Vec<u8>,
    value: Vec<u8>,
}

struct RaftServer {
    sender: mpsc::Sender<Msg>,
}

impl Clone for RaftServer {
    fn clone(&self) ->Self {
        RaftServer {
            sender: self.sender.clone(),
        }
    }
}

impl RaftServer {
    fn new(sender: mpsc::Sender<Msg>) ->RaftServer {
        RaftServer {
            sender,
        }
    }
}
impl Commander for RaftServer {
    fn send_command(&mut self, ctx: RpcContext,
                req: Command, 
                sink: UnarySink<CommandReply>) {
        println!("Recive a req: {:?}", req);
        //let mut resp = HelloReply::new();
        let resp = apply_command(self.sender.clone(), req.clone());
        
        let f = sink.success(resp)
                .map_err(move |e| eprintln!("Fail to reply {:?}: {:?}", req, e));
        ctx.spawn(f);
    }
    
     
}

impl rafter::Rafter for RaftServer {
    fn send_msg(&mut self, ctx: RpcContext,
                req: Message, 
                sink: UnarySink<Message>) {
        println!("Recive a req: {:?}", req);
        //let mut resp = HelloReply::new();
        let resp = Message::new();
        //let mut resp = apply_command(self.sender.clone(), req.clone());
        apply_message(self.sender.clone(), req.clone());
        
        let f = sink.success(resp)
                .map_err(move |e| eprintln!("Fail to reply {:?}: {:?}", req, e));
        ctx.spawn(f);
    }
}

fn store_commnad(rocks_db: &DB, raft_command: RaftCommand) -> CommandReply{
    let mut command_reply = CommandReply::new();
    match raft_command.op {
        0 => {
            match rocks_db.put(&raft_command.key[..], &raft_command.value[..]) {
                Ok(_) =>{ 
                    command_reply.set_ok(true);
                },
                Err(e) =>{ 
                    command_reply.set_ok(false);
                    command_reply.set_value(e.as_bytes().to_vec());
                },
            }
        },
        1 => {
            match rocks_db.delete(&raft_command.key[..]) {
                Ok(_) =>{ 
                    command_reply.set_ok(true);
                },
                Err(e) =>{ 
                    command_reply.set_ok(false);
                    command_reply.set_value(e.as_bytes().to_vec());
                },
            }
        },
        2 => {
            match rocks_db.get(&raft_command.key[..]) {
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
        _ => {
            command_reply.set_ok(false);
            command_reply.set_value( b"Error command".to_vec());
        },
    }
    println!("Command reply value is {:?}", command_reply.get_value());
    command_reply
}

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
    let port = 8080 + 2 * port_num;

    


    //start raft
    let rocks_db: DB = DB::open_default("/path/for/rocksdb/storage").unwrap();
    let storage = MemStorage::new();
    let cfg = Config{
        id: port_num,
        peers: vec![1],
        election_tick: 10,
        heartbeat_tick: 3,
        max_size_per_msg: 1024 * 1024 * 1024,
        max_inflight_msgs: 256,
        applied: 0,
        tag: format!("[{}]", 1),
        ..Default::default()
    };
    let mut peers = vec![];
    let peer_count = num+1;
    for i in 1..peer_count {
        let peer_ip = format!("127.0.0.1:{}", 8080 + 2 * i);
        peers.push(Peer {
            id: i as u64,
            context: Some(peer_ip.as_bytes().to_vec()),
        });
    }

    let mut r = RawNode::new(&cfg, storage, peers).unwrap();
    let (sender, receiver) = mpsc::channel();
    println!("send propose!");
    let env = Arc::new(Environment::new(1));
    let raft_server = RaftServer::new(sender);
    let service = raftpb_grpc::create_commander(raft_server.clone());
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind("127.0.0.1", port as u16)
        .build().unwrap();
    
    server.start();

    let service = rafter::create_rafter(raft_server.clone());
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", port as u16)
        .build().unwrap();
    
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("Listening on {}:{}", host, port);
    }

    // send_propose(sender);
    let mut t = Instant::now();
    let mut timeout = Duration::from_millis(100);
    let mut cbs = HashMap::new();
    let mut peer_ips = HashMap::new();
    let env = Arc::new(EnvBuilder::new().build());

    loop {
        match receiver.recv_timeout(timeout) {
            Ok(Msg::Propose { id, command, cb }) => {
                let is_leader = r.raft.leader_id == r.raft.id;
                if is_leader {
                    println!("is leader");
                    cbs.insert(id, cb);
                    let raft_command = RaftCommand {
                        op: command.command_type.value(),
                        key: command.key,
                        value: command.value,
                    };
                    let data = serde_json::to_string(&raft_command).unwrap();
                    println!("Call Data is: {}", data);
                    r.propose(data.as_bytes().to_vec(), vec![id]).unwrap();
                } else {
                    println!("is not leader");
                }
            },
            Ok(Msg::Raft(m)) => r.step(m).unwrap(),
            Err(RecvTimeoutError::Timeout) => (),
            Err(RecvTimeoutError::Disconnected) => return,
        }

        let d = t.elapsed();
        if d >= timeout {
            t = Instant::now();
            timeout = Duration::from_millis(100);
            // We drive Raft every 100ms.
            println!("raft tick: State is {:?}", r.raft.state);
            r.tick();
        } else {
            timeout -= d;
        }

        on_ready(&mut r, &mut cbs, &mut peer_ips, env.clone(), &rocks_db);
    }
    
}

fn on_ready(r: &mut RawNode<MemStorage>,
            cbs: &mut HashMap<u8, ProposeCallback>, 
            peers_clients: &mut HashMap<u64, Box<rafter::RafterClient>>,
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
        let mut fvec = vec![];
        for msg in msgs {
            let to = msg.get_to();
            if to != r.raft.id {
                let client = peers_clients.get(&to).unwrap();
                fvec.push(client.send_msg_async(&msg).unwrap()
                    .and_then(move |mut resp| {
                        println!("value is {:?}", resp);
                        Ok(())
                    }).map_err(|_| ()));
            }
            // Here we only have one peer, so can ignore this.
        }
        for f in fvec {
            tokio::run(f);
        }
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
        r.mut_store().wl().set_hardstate(hs.clone());
    }

    if !is_leader {
        // If not leader, the follower needs to reply the messages to
        // the leader after appending Raft entries.
        let msgs = ready.messages.drain(..);
        println!("ready messages is {:?}", msgs);
        let mut fvec = vec![];
        for msg in msgs {
            let to = msg.get_to();
            if to != r.raft.id {
                let client = peers_clients.get(&to).unwrap();
                fvec.push(client.send_msg_async(&msg).unwrap()
                    .and_then(move |mut resp| {
                        println!("value is {:?}", resp);
                        Ok(())
                    }).map_err(|_| ()));
            }
            // Here we only have one peer, so can ignore this.
        }
        for f in fvec {
            tokio::run(f);
        }
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
                println!("Data is {:?}", entry.context.as_slice());
                if let Some(cb) = cbs.remove(entry.get_data().get(0).unwrap()) {
                    let raft_command: RaftCommand = serde_json::from_slice(entry.context.as_slice()).unwrap();
                    let reply = store_commnad(rocks_db, raft_command);
                    println!("start call back");
                    cb(reply);
                }
            } else {
                use protobuf::Message;
                println!("Entry is: {:?}", entry);
                let mut cc = ConfChange::new();
                cc.merge_from_bytes(&entry.get_data());
                println!("ConfChange: {:?}", cc);
                println!("Append ip is {}", String::from_utf8(cc.get_context().to_vec()).unwrap());
                match cc.get_change_type() {
                    ConfChangeType::RemoveNode => {
                        peers_clients.remove(&cc.get_node_id());
                    },
                    _ => {
                        if cc.get_node_id() != r.raft.id {
                            let ch = ChannelBuilder::new(env.clone()).connect(
                                    &String::from_utf8(cc.get_context().to_vec()).unwrap());
                            let client = rafter::RafterClient::new(ch);
                            peers_clients.insert(cc.get_node_id(), Box::new(client));
                        }
                    }
                }
            }

            // TODO: handle EntryConfChange
        }
    }

    // Advance the Raft
    r.advance(ready);
}

fn send_propose(sender: mpsc::Sender<Msg>) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(10));
        let (s1, r1) = mpsc::channel::<u8>();
        println!("propose a request");
        sender
            .send(Msg::Propose {
                id: 1,
                command: Command::new(),
                cb: Box::new(move |_relpy| {
                    s1.send(0).unwrap();
                }),
            }).unwrap();
        let n = r1.recv().unwrap();
        assert_eq!(n, 0);
        println!("receive the propose callback");
    });
}



fn apply_command(sender: mpsc::Sender<Msg>, command: Command) 
    ->CommandReply {
    let (s1, r1) = mpsc::channel::<CommandReply>();

    println!("Propose a command");

    sender.send(Msg::Propose {
        id: 1,
        command,
        cb: Box::new(move |reply| {
            s1.send(reply).unwrap();
        }),
    }).unwrap();
    
    let reply = r1.recv().unwrap();

    println!("Receive the commnad reply");
    reply
}

fn apply_message(sender: mpsc::Sender<Msg>, message: Message) {
    println!("Propose a command");

    sender.send(Msg::Raft(message)).unwrap();

    println!("Receive the commnad reply");
}