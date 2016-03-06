extern crate mio;
extern crate bytes;

#[doc="

"]

use super::chatter::{Chatter};
use super::message::{MessageType, Message};
use std::io::{self, Read};
use std::collections::HashMap;
use super::types::Id;

use self::mio::{TryRead, TryWrite};
use self::mio::tcp::TcpStream;
use self::mio::util::Slab;
use self::bytes::Buf;

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
    server: SocketAddr,
    //TODO: put in the game client here
}

impl ChatClient {
    pub fn new(server: SocketAddr) -> ChatClient {
        ChatClient {
            server: server,
        }
    }

    pub fn start() {
        unimplemented!();
    }

    fn select_game() {
        unimplemented!();
    }
}
