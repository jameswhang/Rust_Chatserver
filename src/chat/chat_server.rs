extern crate mio;
extern crate bytes;

use super::types::*;
use std::collections::HashMap;
use super::chatter::{Chatter};
use super::chat_room::*;
use mio::tcp::*;
use mio::util::Slab;

pub struct ChatServer {
    server: TcpListener,
	active_users: UserMap,  // maintains a map of all users
	chat_rooms : RoomMap,
}


// This struct represents the status of a single connection to the client
pub struct ConnectionStatus {
    socket: TcpStream, // tcp socket
    token: mio::Token, // MIO token that registers the socket with EventLoop
    state: State,
}

impl ChatServer {
    // Initializing a server from a provided TCP socket
	pub fn new(server: TcpListener) -> ChatServer {
		ChatServer {
            server: server,
			active_users : UserMap::new(),
			chat_rooms : RoomMap::new(),
		}
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

        match token {
            SERVER => {
                // server socket is ready to be operated

                assert!(event.is_readable());

                match self.server.accept() {
                    // new client connection has been established
                    Ok(Some(socket)) => {
                        println!("accepted a new client socket");
                        let token = self.active_users.insert_with(|token| ConnectionStatus::new(socket, token)).unwrap();

                        // This basically registers the connection with event loop.
                        // We only request readable events.
                        // Read a full line of input before attemtping to write
                        // to the socket so that we don't wanna receive "writable"
                        // events yet since there is nothing to do
                        event_loop.register_opt(
                            &self.active_users[token].socket,
                            token,
                            mio::EventSet::readable(),
                            mio::PollOpt::edge() | mio::PollOpt::oneshot()
                        ).unwrap();
                    }

                    Ok(None) => {
                        // This case is if the server socket is not ready yet
                        println!("The server socket is not ready yet");
                    }
                    Err(e) => {
                        // Something unexpected happening.. 
                        // TODO: Error handling
                        println!("Some error while server was accepting connection; err={:?}", e);
                        event_loop.shutdown();
                    }
                }
            }
            _ => {
                self.active_users[token].ready(event_loop, events);
                // If ahndling event resulted in a closed socket, then
                // remove the socket from the Slab and free resource
                if self.connections[token].is_closed() {
                    let _ = self.connections.remove(token);
                }
            }
        }
    }
}

impl ConnectionStatus {
    fn new(socket: TcpStream, token: mio::Token) -> ConnectionStatus {
        ConnectionStatus {
            socket: socket,
            token: token, 
            state: State::Reading(vec![]),
        }
    }

    fn ready(&mut self, event_loop: &mut mio::EventLoop<ChatServer>, events: mio::EventSet) {
        println!("      connection-state={:?}", self.state);
        match self.state {
            State::Reading(..) => {
                assert!(events.is_readable(), "unexpected events; events={:?}", events);
                self.read(event_loop)
            }
            State::Writing(..) => {
                assert!(events.is_writable(), "unexpected events; events={:?}", events);
            }
            _ => unimplemented!(),
        }
    }

    fn read(&mut self, event_loop: &mut mio::EventLoop<ChatServer>) {
        match self.socket.try_read_buf(self.state.mut_read_buf()) {
            Ok(Some(0)) => {
                println!("      read 0 bytes from client; buffered={}",
                         self.state.read_buf().len());

                match self.state.read_buf().len() {
                    n if n > 0 => {
                        // Transition to a writing state even if a new line hasn't been received
                        self.state.transition_to_writing(n);
                        // Re-registering the socket with the event loop
                        // This notifies us when the socket becomes writable.
                        self.reregister(event_loop);
                    }
                    // TODO: fix this
                    _ => {
                        self.state = State::Closed;
                    }
                }
            }
            Ok(Some(n)) => {
                println!("      read {} bytes", n);
                // Look for a new line, and if it is there, 
                // then state transition from reading to writing
                self.state.try_transition_to_writing();
                self.reregister(event_loop);
            }
            Ok(None) => {
                self.reregister(event_loop);
            }
            Err(e) => {
                panic!("Got an error while trying to read; err={:?}", e);
            }
        }
    }

    fn write(&mut self, event_loop: &mut mio::EventLoop<ChatServer>) {
        //TODO: error handling
        match self.socket.try_write_buf(self.state.mut_write_buf()) {
            Ok(Some(_)) => {
                // If the entire line has been rewritten, transition back to the
                // reading state
                self.state.try_transition_to_reading();

                // Re-register the socket with the event loop
                self.reregister(event_loop);
            }
            Ok(None) => {
                // The socket wasn't actually ready, re-register the socket
                // with the event loop
                self.reregister(event_loop);
            }
            Err(e) => {
                panic!("Got an error trying to write; err={:?}", e);
            }
        }
    }
    fn reregister(&self, event_loop: &mut mio::EventLoop<ChatServer>) {
        // Maps current client state to the mio 'EventSet' that will provide
        // with us the notifications we want. When we are currently reading 
        // from the client, we want 'readable' socket notifications. 
        // When we are writing to the client, we want 'writable' notifications
        let event_set = match self.state {
            State::Reading(..) => mio::EventSet::readable(),
            State::Writing(..) => mio::EventSet::writable(),
            _ => mio::EventSet::none(),
        };
        event_loop.reregister(&self.socket, 
                               self.token, 
                               event_set,
                               mio::PollOpt::oneshot()
                             ).unwrap();
    }
    fn is_closed(&self) -> bool {
        match self.state {
            State::Closed => true,
            _ => false,
        }
    }
}

impl State {
    fn mut_read_buf(&mut self) -> &mut Vec<u8> {
        match *self {
            State::Reading(ref mut buf) => buf,
            _ => panic!("Connection not in reading state"),
        }
    }
    fn read_buf(&self) -> &[u8] {
        match *self {
            State::Reading(ref buf) => buf,
            _ => panic!("Connection not in reading state"),
        }
    }

    fn write_buf(&self) -> &Take<Cursor<Vec<u8>>> {
        match *self {
            State::Writing(ref buf) => buf,
            _ => panic!("Connection not in writing state"),
        }
    }
    fn mut_write_buf(&self) -> &mut Take<Cursor<Vec<u8>>> {
        match *self {
            State::Writing(ref mut buf) => buf,
            _ => panic!("Connection not in writing state"),
        }
    }

    // Looks for a new line, if there is one the state is transitioned to
    // writing
    fn try_transition_to_writing(&mut self) {
        // Looks for newline
        if let Some(pos) = self.read_buf().iter().position(|b| *b == b'\n') {
            self.transition_to_string(pos + 1);
        }
    }

    fn transition_to_writing(&mut self, pos:usize) {
        // First, remove the current read buffer, placing it with an
        // empty Vec<u8>
        let buf = mem::replace(self, State::Closed).unwrap_read_buf();

        // Wrap in 'Cursor' this allows Vec<u8> to act as a readable buffer
        let buf = Cursor::new(buf);

        // Transition the state to 'writing', limiting the buffer to the new line
        *self = State::Writing(Take::new(buf, pos));
    }

    fn try_transition_to_reading(&mut self) {
        if !self.write_buf().has_remaining() {
            let cursor = mem::replace(self, State::Closed).unwrap_write_buf().into_inner();
            let pos = cursor.position();
            let mut buf = cursor.into_inner();

            // Drop all data that has been written to the client
            drain_to(&mut buf, pos as usize);

            *self = State::Reading(buf);
            
            // Check for any new lines that have already been read
            self.try_transition_to_writing();
        }
    }
    fn unwrap_read_buf(self) -> Vec<u8> {
        match self {
            State::Reading(buf) => buf,
            _ => panic!("Connection is not in the reading state"),
        }
    }

    fn unwrap_write_buf(self) -> Take<Cursor<Vec<u8>>> {
        match self {
            State::Writing(buf) => buf,
            _ => panic!("Connection is not in the writing state"),
        }
    }
}

pub fn start(address: SocketAddr) {
    // Create a new non-blocking socket bound to the given address. All sockets
    // created by mio are set to non-blocking mode.
    let server = TcpListener::bind(&address).unwrap();

    // Create a new mio EventLoop
    let mut event_loop = mio::EventLoop::new().unwrap();

    // Register the server's socket with the event loop
    event_loop.register(&register, SERVER).unwrap();

    // Create a new ChatServer instance that trakcs the state of the server
    let mut cs = ChatServer::new(server);

    // Run the ChatServer
    println!("Running the chatserver; port = 6567");
    event_loop.run(&mut cs).unwrap();
}

fn main() {
    start("0.0.0.0:6567".parse().unwrap());
}

fn drain_to(vec: &mut Vec<u8>, count: usize) {
    for _ in 0..count {
        vec.remove(0);
    }
}




#[cfg(test)]
mod test {
    use std::io::{BufRead, BufReader, Read, Write};
    use std::net::{Shutdown, TcpStream};

    #[test]
    pub fn test_basic_echoing() {
        start_server();

        let mut sock =
            BufReader::new(TcpStream::connect(HOST).unwrap());
        let mut recv = String::new();

        sock.get_mut().write_all(b"hello world\n").unwrap();
        sock.read_line(&mut recv).unwrap();

        assert_eq!(recv, "hello world\n");

        recv.clear();

        sock.get_mut().write_all(b"this is a line\n").unwrap();
        sock.read_line(&mut recv).unwrap();

        assert_eq!(recv, "This is a line\n");
    }

    #[test]
    pub fn test_handling_client_shutdown() {
        start_server();

        let mut sock = TcpStream::connect(HOST).unwrap();

        sock.write_all(b"hello world").unwrap();
        sock.shutdown(Shutdown::Write).unwrap();

        let mut recv = vec![];
        sock.read_to_end(&mut recv).unwrap();

        assert_eq!(recv, b"hello world");
    }

    const HOST: &'static str = "0.0.0.0:13254";

    fn start_server() {
        use std::thread;
        use std::sync::{Once, ONCE_INIT};

        static INIT: Once = ONCE_INIT;

        INIT.call_once(|| {
            thread::spawn(|| {
                super::start(HOST.parse().unwrap())
            });

            while
            let Err(_) =
            TcpStream::connect(HOST) {
                // Loop
            }
        });
    }
}










