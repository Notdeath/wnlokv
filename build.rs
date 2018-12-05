extern crate protoc_grpcio;

fn main() {
    let proto_root = "protos";
    println!("cargo:rerun-if-changed={}", proto_root);
    protoc_grpcio::compile_grpc_protos(
        &["raftpb.proto"],
        &[proto_root],
        &proto_root).expect("Fail to compile gRpc");
}
