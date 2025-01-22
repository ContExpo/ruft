//Macros
#![allow(unused_parens)]


use std::{env::args, process::exit};

mod raft;

fn main() {
    let argv = std::env::args();
    let argc = argv.count();
    if (argc < 3) {
        print!("Expected usage: program <list of IP:port replicas> <id of this replica>");
        println!("At least one replica is expected");
        exit(-1);
    }
    let replicas: Vec<String> = args().collect();
    let replicas= &replicas[1..replicas.len() - 1];
    let replica_idS = std::env::args().nth(1);
    let replica_id: u32;
    match(replica_idS){
        Some(x) => replica_id = x.parse(),
        None => todo!(),
    }

    

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}