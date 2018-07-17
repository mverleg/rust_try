#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde;
extern crate serde_json;

mod callee;
mod caller;
mod iface;
mod rest_client;
mod rest_server;

fn main() {
    println!("Hello, world!!");
}
