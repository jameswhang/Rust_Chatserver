#[doc="

"]

use super::chatter::{Chatter};
use super::message::{MessageType, Message};
use std::io::{self, Read};
use std::collections::HashMap;
use std::net::{TcpStream, SocketAddr};

pub type Id = String;

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

	pub fn read(&mut self) {
		let input: String;
		loop {
			input = read_from_stream(&mut self.connection);
			if input.len() > 0 {
				let message = self.receive_message(input);
				// do something with message
			}
		}
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

pub fn read_from_stream(stream: &mut TcpStream) -> String {
    const BUF_SIZE: usize = 128;
    let mut buf = [0; BUF_SIZE];
    let mut result = String::new();
    let mut addition: String;

    // continually pass in a buffer until nothing left to read
    while let Ok(length) = stream.read(&mut buf[..]) {
        // add data in buffer to results string
        addition = String::from_utf8(buf.to_owned()).unwrap();
        result.push_str(&addition);
        buf = [0; BUF_SIZE];

        // break if all of input has been read
        if length < BUF_SIZE {
            break;
        }
    }

    result
}
