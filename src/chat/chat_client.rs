extern crate mio;
extern crate bytes;

#[doc="

"]

use super::message::{MessageType, Message};

use std::io::prelude::*;
use std::net::TcpStream;

use super::chat_server::ChatServer;

use std::{mem, str};
use std::io::Cursor;
use std::net::SocketAddr;


#[derive(Debug, PartialEq)]
pub enum ClientStatus {
	SelectingAction,
	SelectingRoom,
    LeavingRoom,
	InRoom,
}


// server client.
pub struct ChatClient {
    stream: TcpStream,
}

impl ChatClient {
    pub fn new(server_addr: SocketAddr) -> ChatClient {
        ChatClient {
            stream: TcpStream::connect(&server_addr).unwrap()
        }
    }

    fn show_all_rooms(&mut self) {
        self.send_msg(format!("SHOWROOMS"));
        let mut buf = [0u8; 8];
        self.stream.read(&mut buf).unwrap();
    }

    pub fn send_msg(&mut self, msg: String) {
        let mut buf = [0u8; 8]; // Some complications exist with the interaction between
                                // mio/kqueue. Refer to chat_server for more explanation
        self.stream.write_all(buf.as_ref()).unwrap();
        self.stream.write_all(msg.as_ref()).unwrap();
    }

    fn select_game() {
        unimplemented!();
    }
}
