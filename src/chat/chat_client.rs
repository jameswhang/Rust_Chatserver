#[doc="

"]

use super::chatter::{Chatter};
use std::io::{self, Read};
use std::collections::HashMap;
use std::net::{TcpStream, SocketAddr};


pub struct ChatClient {
	server_addr : SocketAddr,
	chatter : Chatter,
}

impl ChatClient {
	pub fn new(server_addr : SocketAddr, chatter: Chatter) -> ChatClient {
		ChatClient {
			server_addr : server_addr,
			chatter: chatter,
		}
	}

	pub fn start(server_address : SocketAddr) {
		let username = get_username();
        // Setting default server port to 8080
        let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
		let mut chatter = Chatter::new(username, stream);
	}
}

fn get_username() -> String {
	println!("Enter your desired username: ");
	read_username()
}

fn read_username() -> String {
	let mut username = String::new();

    //try!(io::stdin().read_to_string(&mut username));
	username
}
