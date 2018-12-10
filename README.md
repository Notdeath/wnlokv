# README 
 This is a littile  distributed KV server for learn.
 The KV server can deploy at a single mathine.
 It has two major part, the server and the client. The server is a KV server based on 
 grpc-rs, raft-rs and rocksdb-rs. The client is a test for the server.
 
## library
- [rust-rocksdb](https://github.com/pingcap/rust-rocksdb): Our RocksDB binding and wrapper for Rust
- [raft-rs](https://github.com/pingcap/raft-rs): The Raft distributed consensus algorithm implemented in Rust
- [grpc-rs](https://github.com/pingcap/grpc-rs): The gRPC library for Rust built on the gRPC C Core library and Rust Futures

## run
###server
like this
```
  cargo run --bin raft 5 1
```
2 

###client
```
  cargo run --bin client 
```
