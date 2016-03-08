#![allow(dead_code)]
extern crate chitchat;
extern crate mio;
extern crate games;

mod chat;

use chitchat::chat::chat_server::{ChatServer};
use chitchat::chat::chat_client::{ChatClient};
use std::env;
use std::net::{SocketAddr};


use self::mio::EventLoop;
use self::mio::tcp::*;

fn main(){
    let mut args: Vec<_> = env::args().collect();
	if args.len() < 3 {
		panic!("usage: cargo run [mode] [server_ip]:[server_port]");
	}

	let mode = args[1].clone();

	if mode == "s" {
        start_server(args[2].clone());
	} else if mode == "c" {
        let server_address = args[2].clone();
        let server: SocketAddr = server_address.parse().unwrap();
        start_client(server);
        /*
            Err(e) => {
			    panic!("Invalid server address was provided. Use format server_ip:server_port");
            }
		}
        */
	} else {
		panic!("usage: mode has to be either s for server, or c for client");
	}
}

fn start_client(server_address : SocketAddr) {
	let mut client = ChatClient::new(server_address);
}


fn start_server(server_addr : String) {
    let addr = server_addr.parse::<SocketAddr>().ok().expect("Failed to parse host:port");
    let sock = TcpListener::bind(&addr).ok().expect("Failed to bind the address");
    let mut event_loop = EventLoop::new().ok().expect("Failed to create the event loop");
    let mut server = ChatServer::new(sock);

    server.register(&mut event_loop).ok().expect("Failed to register the server with event loop");
    event_loop.run(&mut server).unwrap_or_else(|e| {
        println!("Event loop failed {:?}", e);
    });

}
