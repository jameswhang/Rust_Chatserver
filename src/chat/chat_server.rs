extern crate mio;
extern crate bytes;

use std::rc::Rc;
use std::io;
use std::str;
use std::mem;
use std::collections::HashMap;
use std::net::SocketAddr;

use super::types::*;
use super::chatter::{Chatter};
use super::chat_room::*;

use self::mio::*;
use self::mio::tcp::*;
use self::mio::util::Slab;

use self::bytes::{Buf, Take};
use super::chat_connection::Connection;

pub struct ChatServer {
    sock: TcpListener,
    token: Token,
	connections: Slab<Connection>,  // maintains a map of all connections
}

impl ChatServer {
    // Initializing a server from a provided TCP socket
	pub fn new(sock: TcpListener) -> ChatServer {
		ChatServer {
            sock: sock,
            token: Token(1),
			connections: Slab::new_starting_at(mio::Token(2), 128),
		}
	}

    /// Register the server with event loop
    pub fn register(&mut self, event_loop: &mut EventLoop<ChatServer>) -> io::Result<()> {
        event_loop.register(
            &self.sock,
            self.token,
            EventSet::readable(),
            PollOpt::edge()
        ).or_else(|e| {
            println!("Failed to register the server {:?}, {:?}", self.token, e);
            Err(e)
        })
    }

    pub fn accept(&mut self, event_loop: &mut EventLoop<ChatServer>) {
        println!("Server accepting a new socket!");
        loop {
            // Log an error if there is no socket, but otherwise move on 
            let sock = match self.sock.accept() {
                Ok(s) => {
                    match s {
                        Some((sock, _)) => sock,
                        None => {
                            println!("accept encounted WouldBlock");
                            return;
                        }
                    }
                },
                Err(e) => {
                    println!("Failed to accept a new socket, {:?}", e);
                    return;
                }
            };

            match self.connections.insert_with(|token| {
                println!("registering {:?} with event loop", token);
                Connection::new(sock, token)
            }) {
                Some(token) => {
                    // Insert successful. Register the connection.
                    match self.find_connection_by_token(token).register(event_loop) {
                        Ok(_) => {},
                        Err(e) => {
                            println!("Failed to regsiter {:?} connection with the event loop, {:?}",
                                   token, e);
                            self.connections.remove(token);
                        }
                    }
                },
                None => {
                    println!("Failed to insert connection into slab");
                }
            };
        }
    }

    /// Forward a readable event to an established connection.
    /// Connections are identified by the token provided to us from the event loop
    /// Once read has been finished, push the receive buffer into the all the existing connections
    /// so that we can broadcast
    fn readable(&mut self, token: Token) -> io::Result<()> {
        println!("server conn readable; token={:?}", token);
        while let Some(msg) = try!(self.find_connection_by_token(token).readable()) {
            /*
            let client_msg = msg.clone();
            let client_msg_str = str::from_utf8(&client_msg).unwrap();

            let response = ChatServer::handle_actions(client_msg_str).clone();
            let resp = response.clone();
            let response_bytes = response.as_bytes().to_owned();

            let rc_message = Rc::new(response_bytes);
            */
            let rc_message = Rc::new(msg);
            for c in self.connections.iter_mut() {
                c.send_message(rc_message.clone())
                    .unwrap_or_else(|e| {
                        println!("Failed to queue the msg for {:?}: {:?}", c.token, e);
                        c.mark_reset();
                    });
            }
        }
        Ok(())
    }

    fn handle_actions(request_str: &str) -> &str {
        match request_str {
            "SHOWROOMS" => {
                "testshow"
            }

            "JOINROOM" => {
                "testjoin"
            }

            "CREATEROOM" => {
                "testcreate"
            }

            _ => {
                "nothing"
            }
        }
    }

    fn find_connection_by_token<'a>(&'a mut self, token: Token) -> &'a mut Connection {
        &mut self.connections[token]
    }

	pub fn add_chatter(){ unimplemented!();}
	pub fn add_room(){ unimplemented!();}
	pub fn remove_room(){ unimplemented!();}
	pub fn remove_chatter(){ unimplemented!();}
}

impl mio::Handler for ChatServer {
    type Timeout = (); // timeouts
    type Message = (); // cross thread notifications


    fn ready(&mut self, event_loop: &mut mio::EventLoop<ChatServer>, token: mio::Token, events: mio::EventSet) {
        println!("Socket is ready: token={:?}; events={:?};", token, events);

        if events.is_error() {
            println!("Error event for {:?}", token);
            self.find_connection_by_token(token).mark_reset();
            return;
        }

        if events.is_hup() {
            println!("Hup event for {:?}", token);
            self.find_connection_by_token(token).mark_reset();
            return;
        }

        if events.is_writable() {
            let conn = self.find_connection_by_token(token);

            if conn.is_reset() {
                println!("{:?} has already been reset", token);
            }

            conn.writable().unwrap_or_else(|e| {
                println!("Write event failed for {:?}, {:?}", token, e);
                conn.mark_reset();
            });
        }

        // read event for server is a new connection establishment
        if events.is_readable() {
            if self.token == token {
                self.accept(event_loop);
            } else {
                if self.find_connection_by_token(token).is_reset() {
                    println!("{:?} has already been reset", token);
                    return;
                }
                self.readable(token).unwrap_or_else(|e| {
                    println!("Read event failed for {:?}, {:?}", token, e);
                    self.find_connection_by_token(token).mark_reset();
                });
            }
        }

        if self.token != token {
            self.find_connection_by_token(token).mark_idle();
        }
    }
}
