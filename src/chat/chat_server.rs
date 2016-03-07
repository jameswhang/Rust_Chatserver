extern crate mio;

use std::io;
use std::str;
use std::rc::Rc;
use std::string::String;

use self::mio::*;
use self::mio::tcp::*;
use self::mio::util::Slab;

use super::chat_connection::Connection;
use super::chat_client::ChatClient;
use super::chat_app::ChatApp;
use super::chat_room::ChatRoom;
use super::message::*;
use super::types::*;

pub struct ChatServer {
    sock: TcpListener,
    token: Token,
    conns: Slab<Connection>,
    app: ChatApp,
}

impl<'b> mio::Handler for ChatServer {
    type Timeout = ();
    type Message = ();

    fn tick(&mut self, event_loop: &mut EventLoop<ChatServer>) {
        println!("Handling end of tick");

        let mut reset_tokens = Vec::new();

        for c in self.conns.iter_mut() {
            if c.is_reset() {
                reset_tokens.push(c.token);
            } else if c.is_idle() {
                c.reregister(event_loop)
                    .unwrap_or_else(|e| {
                        println!("Reregister failed {:?}", e);
                        c.mark_reset();
                        reset_tokens.push(c.token);
                    });
            }
        }

        for token in reset_tokens {
            match self.conns.remove(token) {
                Some(_c) => {
                    println!("reset connection; token={:?}", token);
                }
                None => {
                    println!("Unable to remove connection for {:?}", token);
                }
            }
        }
    }

    fn ready(&mut self, event_loop: &mut EventLoop<ChatServer>, token: Token, events: EventSet) {
        println!("{:?} events = {:?}", token, events);
        assert!(token != Token(0), "[BUG]: Received event for ChatServer token {:?}", token);

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

        // We never expect a write event for our `ChatServer` token . A write event for any other token
        // should be handed off to that connection.
        if events.is_writable() {
            println!("Write event for {:?}", token);
            assert!(self.token != token, "Received writable event for ChatServer");

            let conn = self.find_connection_by_token(token);

            if conn.is_reset() {
                println!("{:?} has already been reset", token);
                return;
            }

            conn.writable()
                .unwrap_or_else(|e| {
                    println!("Write event failed for {:?}, {:?}", token, e);
                    conn.mark_reset();
                });
        }

        // A read event for our `ChatServer` token means we are establishing a new connection. A read
        // event for any other token should be handed off to that connection.
        if events.is_readable() {
            println!("Read event for {:?}", token);
            if self.token == token {
                self.accept(event_loop);
            } else {

                if self.find_connection_by_token(token).is_reset() {
                    println!("{:?} has already been reset", token);
                    return;
                }

                self.readable(token)
                    .unwrap_or_else(|e| {
                        println!("Read event failed for {:?}: {:?}", token, e);
                        self.find_connection_by_token(token).mark_reset();
                    });
            }
        }

        if self.token != token {
            self.find_connection_by_token(token).mark_idle();
        }
    }
}

impl<'b> ChatServer {
    // Initializing a server from a provided TCP socket
	pub fn new(sock: TcpListener) -> ChatServer {
		ChatServer {
            sock: sock,
            // I don't use Token(0) because kqueue will send stuff to Token(0)
            // by default causing really strange behavior. This way, if I see
            // something as Token(0), I know there are kqueue shenanigans
            // going on.
            token: Token(1),
			conns: Slab::new_starting_at(mio::Token(2), 128),
            app: ChatApp::new(),
		}
	}

    /// Register ChatServer with the event loop.
    ///
    /// This keeps the registration details neatly tucked away inside of our implementation.
    pub fn register(&mut self, event_loop: &mut EventLoop<ChatServer>) -> io::Result<()> {
        event_loop.register(
            &self.sock,
            self.token,
            EventSet::readable(),
            PollOpt::edge()
        ).or_else(|e| {
            println!("Failed to register server {:?}, {:?}", self.token, e);
            Err(e)
        })
    }

    ///
    /// The server will keep track of the new connection and forward any events from the event loop
    /// to this connection.
    fn accept(&mut self, event_loop: &mut EventLoop<ChatServer>) {
        println!("server accepting new socket");

        loop {
            // Log an error if there is no socket, but otherwise move on
            let sock = match self.sock.accept() {
                Ok(s) => {
                    match s {
                        Some((sock, _)) => sock,
                        None => {
                            println!("accept encountered WouldBlock");
                            return;
                        }
                    }
                },
                Err(e) => {
                    println!("Failed to accept new socket, {:?}", e);
                    return;
                }
            };

            match self.conns.insert_with(|token| {
                println!("registering {:?} with event loop", token);
                Connection::new(sock, token)
            }) {
                Some(token) => {
                    // If we successfully insert, then register our connection.
                    match self.find_connection_by_token(token).register(event_loop) {
                        Ok(_) => {},
                        Err(e) => {
                            println!("Failed to register {:?} connection with event loop, {:?}", token, e);
                            self.conns.remove(token);
                        }
                    }
                },
                None => {
                    // If we fail to insert, `conn` will go out of scope and be dropped.
                    println!("Failed to insert connection into slab");
                }
            };
        }
    }

    /// Forward a readable event to an established connection.
    ///
    /// Connections are identified by the token provided to us from the event loop. Once a read has
    /// finished, push the receive buffer into the all the existing connections so we can
    /// broadcast.
    fn readable(&mut self, token: Token) -> io::Result<()> {
        println!("server conn readable; token={:?}", token);

        while let Some(message) = try!(self.find_connection_by_token(token).readable()) {
            let msg = message.clone();
            let msg_string = str::from_utf8(&msg).unwrap();
            let handler_request = msg_string.clone().to_string();
            
            // get appropriate response from app
            let mut resp = self.app.handle_server_message(token, handler_request);

            for tok in resp.clients.iter() {
                self.write_to_client_stream(*tok, resp.message.payload().clone());
            }
        }

        Ok(())
    }

    pub fn write_to_client_stream(&mut self, token: mio::Token, response: String) {
        let mut c = self.find_connection_by_token(token);
        let resp_bytes = (&*response).as_bytes().to_owned();
        let rc_message = Rc::new(resp_bytes);
        c.send_message(rc_message.clone())
            .unwrap_or_else(|e| {
                println!("Failed to queue message for {:?}: {:?}", c.token, e);
                c.mark_reset();
            });
    }

    /// Find a connection in the slab using the given token.
    fn find_connection_by_token<'a>(&'a mut self, token: Token) -> &'a mut Connection {
        &mut self.conns[token]
    }
}
