#[doc="

"]

use super::chatter::{Chatter};
use std::io::{self, Read};


pub struct ChatClient {
	server_addr : SocketAddr,
	chatter : Chatter,
}

impl ChatClient {
	pub fn new(server_addr : SocketAddr, chatter: Chatter) -> ChatClient {
		ChatClient {
			server_addr : server_addr,
			chatter: Chatter,
		}
	}

	pub fn start(server_address : SocketAddr) {
		let username = get_username();
		let mut chatter = Chatter::new();
	}
}

fn get_username() -> String {
	println!("Enter your desired username: ");
	read_username()
}

fn read_username() -> String {
	let mut username = String::new();

	while let Err(_) = try!(io::stdin().read_to_string(&mut username)) {}
	username
}
