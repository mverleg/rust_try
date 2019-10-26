
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_macros)]
use std::{fs,io,env};
use std::fs::File;
use std::io::prelude::*;
use std::path::{PathBuf,Path};
use std::collections::HashMap;
use std::time::Duration;
use std::thread;

macro_rules! debug {
    ($x:expr) => {
        println!("{} = {:?}",stringify!($x),$x);
    }
}
extern crate rand;
use rand::*;


fn run(args: Vec<String>) -> std::result::Result<(),Box<dyn std::error::Error+Sync+Send>> {
    println!("{:?}",thread_rng().gen::<[u8; 32]>());
    Ok(())
}
fn main() {
    if let Err(e) = run(std::env::args().collect()) {
        println!("error: {:?}",e);
    }
}
