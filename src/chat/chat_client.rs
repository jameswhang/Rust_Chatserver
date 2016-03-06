#[doc="

"]

use super::chatter::{Chatter};
use super::message::{MessageType, Message};
use std::io::{self, Read};
use std::collections::HashMap;
use std::net::{TcpStream, SocketAddr};
use super::types::Id;


#[derive(Debug, PartialEq)]
pub enum ClientStatus {
	SelectingAction,
	SelectingRoom,
    LeavingRoom,
	InRoom,
}


#[derive(Debug, PartialEq)]
pub enum ActionStatus {
	OK,
	Invalid,
	Failed,
}

pub struct ChatClient {
	username: Id,
	connection: TcpStream,
	status: ClientStatus,
}

impl ChatClient {
	pub fn new(username: String) -> ChatClient {
		// default port 8080 for now
		let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
		let username = get_username();
		println!("New client connected!");

		ChatClient {
			username: username as Id,
			connection: stream,
			status: ClientStatus::SelectingAction,
		}
	}

	pub fn send_message(&mut self, message_type: MessageType, message: String) ->
		Result<ActionStatus, ActionStatus> {
			unimplemented!();
		}

	pub fn receive_message(&mut self, message: String) -> Option<Message> {
		unimplemented!();
	}
}

fn get_username() -> String {
	println!("Enter your desired username: ");
	read_username()
}

fn read_username() -> String {
	let mut username = String::new();

	match io::stdin().read_to_string(&mut username) {
		Ok(_) => {
			username
		},
		Err(_) => {
			"Unknown User".to_string()
		}
	}
}
