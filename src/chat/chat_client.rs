extern crate byteorder;
extern crate chrono;

#[doc="

"]

use super::message::{MessageType, Message};
use super::message::MessageType::*;
use super::types::*;

use std::io::prelude::*;
use std::io;
use std::net::TcpStream;
use self::byteorder::{ByteOrder, BigEndian};
use self::chrono::{UTC};

use super::chat_server::ChatServer;

use std::{mem, str};
use std::io::Cursor;
use std::net::SocketAddr;
use std::time::Duration;


#[derive(Debug, PartialEq)]
pub enum ClientStatus {
	Action,
	SelectingRoom,
    LeavingRoom,
	InRoom,
}


// server client.
pub struct ChatClient {
    stream: TcpStream,
    pub id: Id,
	state: ClientStatus
}

impl ChatClient {
    pub fn new(server_addr: SocketAddr) -> ChatClient {
		let conn = TcpStream::connect(&server_addr).unwrap();
		// conn.set_read_timeout(Some(Duration::from_millis(500)));

        ChatClient {
            stream: conn,
            id: "".to_string(),
            state: ClientStatus::SelectingRoom,
        }
    }

    pub fn select_room(&mut self) {
		let msg = Message::new("BADMID".to_string(), UTC::now(), self.id.clone(), "SERVER".to_string(), MessageType::Show, "".to_string());
        self.send_msg(msg.to_string());
        let mut buf = [0u8; 2048];

		if let Some(raw_message) = self.read_msg() {
			if let Some(message) = Message::from_string(&raw_message) {

				//have the message, make sure it has to do with rooms
				match message.message_type() {
					MessageType::Show => {
						//print out list of rooms
						let mut counter = 1;
						for room in message.payload().lines() {
							println!("{}) {}", counter, room);
						}

						//have user select room
						println!("\nType in desired room name from list:");
						let mut room_choice = String::new();
						let stdin = io::stdin();
						stdin.read_line(&mut room_choice);
						room_choice = room_choice.trim().to_lowercase();

						loop {
							//check if it's in local list
							if let Some(roomname) = message.payload().lines().find(|&x| x == &*room_choice) {
								//send a message to join it
								let req = Message::new("BADMID".to_string(), UTC::now(), self.id.clone(), "SERVER".to_string(), MessageType::Join, roomname.to_string());
								self.send_msg(req.to_string());

								if let Some(response_str) = self.read_msg() {
									if let Some(response) = Message::from_string(&response_str) {
										//check if the server confirms the join
										if response.message_type() == MessageType::Confirm("BADMID".to_string()) {
											println!("Room join confirmed - {}", response.payload());
											self.state = ClientStatus::InRoom;
											break;
										}
									}
								}
							} else {
								println!("That room was not found. Please enter a valid room:");
							}

							stdin.read_line(&mut room_choice);
							room_choice = room_choice.trim().to_lowercase();
						}
					},

					//assume that we are not allowed to look at the rooms currently
					//meaning we need to exit first to see rooms
					MessageType::Reject(_) => {
						println!("{:?}", message.payload());
						self.state = ClientStatus::InRoom;
					},

					_ => {;},
				}
			}
		}
    }

	pub fn start(&mut self) {
		self.set_id();

		loop {
			match self.state {
				ClientStatus::InRoom => {

				},

				ClientStatus::Action => {

				},

				ClientStatus::LeavingRoom => {

				},

				ClientStatus::SelectingRoom => {
					println!("Retrieving rooms...");
					self.select_room();
				},
			}
		}
	}



	fn handle_in_room(&mut self) {
		unimplemented!();
	}

    pub fn send_msg(&mut self, msg: String) {
        let mut buf = [0u8; 8]; // Some complications exist with the interaction between
                                // mio/kqueue. Refer to chat_server for more explanation

        BigEndian::write_u64(&mut buf, msg.len() as u64);

        self.stream.write_all(buf.as_ref()).unwrap();
        self.stream.write_all(msg.as_ref()).unwrap();
    }

	pub fn read_msg(&mut self) -> Option<String> {
		let mut buf = [0u8; 8];
        self.stream.read(&mut buf).unwrap();

        let msg_len = BigEndian::read_u64(&mut buf);
        println!("Reading message length of {}",  msg_len);

        let mut r = [0u8; 256];
        let s_ref = <TcpStream as Read>::by_ref(&mut self.stream);

        match s_ref.take(msg_len).read(&mut r) {
            Ok(0) => {
                println!("0 bytes read");
                None
            },
            Ok(n) => {
                println!("{} bytes read", n);
                let s = str::from_utf8(&r[..]).unwrap();
                println!("read = {}", s);
                Some(s.to_string())
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
	}



	fn set_id (&mut self) {
		println!("Please type in your desired ID: ");
	    let id = &mut String::new();
		let stdin = io::stdin();

		stdin.read_line(id);
		let req = Message::new("BADMID".to_string(), UTC::now(), self.id.clone(), "SERVER".to_string(), MessageType::Connect, id.clone());
		self.send_msg(req.to_string());

		//loop until they get a confirm connect back with their Id
		loop {
			if let Some(raw_message) = self.read_msg() {
				if let Some(message) = Message::from_string(&raw_message) {
					match message.message_type() {
						MessageType::Confirm(_) => {
							println!("{:?}", message.payload());
							self.id = id.clone();
							self.state = ClientStatus::SelectingRoom;
						},

						MessageType::Reject(_) => {
							println!("{:?}", message.payload());
							println!("Input a different ID: ");
							stdin.read_line(id);
							let req = Message::new("BADMID".to_string(), UTC::now(), self.id.clone(), "SERVER".to_string(), MessageType::Connect, id.clone());
							self.send_msg(req.to_string());
						},

						_ => {;},
					}
				}
			}
		}
	}

	fn gen_message(&self, m_type : MessageType, content : &String) -> Message {
		Message::new("BADMID".to_string(), UTC::now(), self.id.clone(), "SERVER".to_string(), m_type, content.clone())
	}

    fn select_game() {
        unimplemented!();
    }
}
