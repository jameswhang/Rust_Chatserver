extern crate chitchat;

use chat::chatServer::{ChatServer};
use chat::chatClient::{ChatClient};
use std::env;

fn main(){
	if env::args.len() < 3 {
		panic!("usage: chitchat mode [server_ip:]server_port"); 
	}

	let mode = env::nth(1).unwrap();

	if mode == "s" {
		unimplemented!("Need to grab local ip address");
	} else if mode == "u" {
		if let Ok(server_address) = SocketAddr::from_str(env::args.nth(2).unwrap()) {
			start_client(server_address);
		} else {
			panic!("Invalid server address was provided. Use format server_ip:server_port");
		}
	} else {
		panic!("usage: mode has to be either s for server, or c for client");
	}
}

fn start_client(server_address : SocketAddr) {
	let client = ChatClient::new(server_address);
	client.start();
}

fn start_server(server_port : String) {
	let server = ChatServer::new(server_port);
	server.start();
}
