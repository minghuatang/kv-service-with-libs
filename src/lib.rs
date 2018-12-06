#![crate_type = "lib"]
#![cfg_attr(test, feature(test))]
extern crate raft;
extern crate grpcio;
extern crate rocksdb;

pub mod server;
pub mod client;
//This module is generated by protobuf.
pub mod kvproto;
