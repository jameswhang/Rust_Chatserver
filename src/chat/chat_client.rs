extern crate byteorder;
extern crate chrono;

#[doc="

"]

use super::message::{MessageType, Message};
use super::types::*;

use std::io::prelude::*;
use std::net::TcpStream;
use self::byteorder::{ByteOrder, BigEndian};
use self::chrono::*;

use super::chat_server::ChatServer;

use std::{mem, str};
use std::io::Cursor;
use std::net::SocketAddr;
use std::time::Duration;


#[derive(Debug, PartialEq)]
pub enum ClientStatus {
	SelectingAction,
	SelectingRoom,
    LeavingRoom,
	InRoom,
}

use ClientStatus::*;

// server client.
pub struct ChatClient {
    stream: TcpStream,
    pub id: Id,
	state: ClientStatus
}

impl ChatClient {
    pub fn new(server_addr: SocketAddr) -> ChatClient {
		let conn = TcpStream::connect(&server_addr).unwrap();
		conn.set_read_timeout(Duration::from_millis(500));

        ChatClient {
            stream: conn,
            id: "".to_string(),
        }
    }

    pub fn show_all_rooms(&mut self) {
		let msg = Message::new("BADMID".to_string(), UTC::now(), self.id.clone(), "SERVER".to_string(), Show, "".to_string());
        self.send_msg(msg.to_string());
        let mut buf = [0u8; 1024];

		//have to loop and block since it has a timeout
		loop {
			if let Ok(size) = self.stream.read(&mut buf) {
				if size > 0 {
					break;
				}
			}
		}
    }

	pub fn start(&mut self) -> {
		self.set_id();
		selfshow_all_rooms

		loop {
			if self.state == InRoom {

			}


		}
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
                println!("0 bytes read", );
            },
            Ok(n) => {
                println!("{} bytes read", n);

                let s = str::from_utf8(&r[..]).unwrap();
                println!("read = {}", s);
            },
            Err(e) => {
                panic!("{}", e);
            }
        }


	}

	fn set_id (&mut self) {
		loop {
			println!("Please type in your desired ID: ");
			let id = stdin();
			self.send_msg(id.to_string());

			//loop until they get a confirm connect back with their Id
			loop {

			}
		}
	}

    fn select_game() {
        unimplemented!();
    }
}
