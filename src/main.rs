extern crate chitchat;
extern crate mio;

mod chat;

use chitchat::chat::chat_server::{ChatServer};
use chitchat::chat::chat_client::{ChatClient};
use std::env;
use std::net::{SocketAddr};

fn main(){
    let mut args: Vec<_> = env::args().collect();
	if args.len() < 3 {
		panic!("usage: cargo run [mode] [server_ip]:[server_port]");
	}


	let mode = args[1].clone();

	if mode == "s" {
		unimplemented!();
	} else if mode == "u" {
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
    unimplemented!();
//	let client = ChatClient::new(server_address);
//	client.start();
}


fn start_server(server_port : String) {
    unimplemented!();
//	let server = ChatServer::new(server_port);
//	server.start();
}
