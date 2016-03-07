extern crate byteorder;

#[doc="

"]

use super::message::{MessageType, Message};
use super::types::*;

use std::io::prelude::*;
use std::net::TcpStream;
use self::byteorder::{ByteOrder, BigEndian};

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
    pub id: Id,
}

impl ChatClient {
    pub fn new(server_addr: SocketAddr, id: Id) -> ChatClient {
        ChatClient {
            stream: TcpStream::connect(&server_addr).unwrap(),
            id: id,
        }
    }

    pub fn show_all_rooms(&mut self) {
        self.send_msg(format!("SHOWROOMS"));
        let mut buf = [0u8; 8];
        self.stream.read(&mut buf).unwrap();
    }

    pub fn send_msg(&mut self, msg: String) {
        let mut buf = [0u8; 8]; // Some complications exist with the interaction between
                                // mio/kqueue. Refer to chat_server for more explanation
        println!("Trying to send message: {:?}", msg);
        println!("Sending over message length of {:?}",  msg.len());

        BigEndian::write_u64(&mut buf, msg.len() as u64);

        self.stream.write_all(buf.as_ref()).unwrap();
        self.stream.write_all(msg.as_ref()).unwrap();

        println!("Done writing");

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

    fn select_game() {
        unimplemented!();
    }
}
