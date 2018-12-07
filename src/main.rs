extern crate rocksdb;
use rocksdb::{DB};
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = &args[1];
    let num: u64 = arg.trim().parse().unwrap();
    let db = DB::open_default(&format!("/path/for/rocksdb/storage{}",num)).unwrap();
    //db.put(b"my key", b"my value");
    for i in 0..100{
    let key = format!("key{}", i);
        match db.get(key.as_bytes()) {
            Ok(Some(value)) => println!("retrieved value {}", value.to_utf8().unwrap()),
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }
    }
}
