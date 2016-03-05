#[doc="

"]

extern crate mio;
extern crate bytes;

use self::mio::{TryRead, TryWrite};
use self::mio::tcp::TcpStream;
use self::mio::util::Slab;
use self::bytes::Buf;

use super::chat_server::ChatServer;

use std::{mem, str};
use std::io::Cursor;
use std::net::SocketAddr;

// server client. Can manage multiple connections.
pub struct ChatClient {
	connections: Slab<Connection>,
}

// connection of a client.
struct Connection {
	socket: TcpStream,
	// token used to register connection with EventLoop
	token: mio::Token,
	// state of connection (reading or writing)
	state: State,
	// remaining lines to send to server
	remaining: Vec<Vec<u8>>,
}

#[derive(Debug)]
enum State {
	Reading(Vec<u8>),
	Writing(Cursor<Vec<u8>>),
	Closed,
}

// ChatClient methods
impl ChatClient {
	fn new() -> ChatClient {
		ChatClient {
			// allocate a slab of arbitrary length. We pick 10 here.
			connections: Slab::new(10),
		}
	}
}

impl mio::Handler for ChatClient {
	fn ready(&mut self, event_loop: &mut mio::EventLoop<ChatServer>, token: mio::Token, events: mio::EventSet) {
		println!("Socket is ready! Token = {:?}; Events = {:?}", token, events);
		self.connections[token].ready(event_loop, events);

		// if a connection is closed, remove it from the slab of connections
		if self.connections[token].is_closed() {
			let _ = self.connections.remove(token);

			// if slab is empty, close the event loop
			if self.connections.is_empty() {
				event_loop.shutdown();
			}
		}
	}
}

// Connection methods
impl Connection {
	fn ready(&mut self, event_loop: &mut mio::EventLoop<ChatServer>, events: mio::EventSet) {
		println!("Connection state = {:?}", self.state);

		// handle event according to the current state of the connection
		match self.state {
			State::Reading(..) => {
				assert!(events.is_readable(), "Unexpected events; events = {:?}", events);
				self.read(event_loop);
			}

			State::Writing(..) => {
				assert!(events.is_writable(), "Unexpected events; events = {:?}", events);
				self.write(event_loop);
			}

			_ => unimplemented!(),
		}
	}

	fn read(&mut self, event_loop: &mut mio::EventLoop<ChatServer>) {
		match self.socket.try_read_buf(self.state.mut_read_buf()) {
			Ok(Some(0)) => {
				// socket seems to be closed, so just update state
				self.state = State::Closed;
			}

			Ok(Some(n)) => {
				println!("Read {} bytes!", n);

				// Read from socket and look for new line
				// if new line exists, transtion to writing state. Otherwise, stay in reading state.
				self.state.try_transition_to_writing(&mut self.remaining);

				//Re-register socket with event loop.
				self.reregister(event_loop);
			}

			Ok(None) => {
				self.reregister(event_loop);
			}

			Err(e) => {
				panic!("Error in client reading; err = {:?}", e);
			}
		}
	}

	fn write(&mut self, event_loop: &mut mio::EventLoop<ChatServer>) {
		match self.socket.try_write_buf(self.state.mut_write_buf()) {
			Ok(Some(_)) => {
				// if the entire buffer has been written, transition to the reading state
				self.state.try_transition_to_reading();

				// Re-register
				self.reregister(event_loop);
			}

			Ok(None) => {
				// Re-register
				self.reregister(event_loop);
			}

			Err(e) => {
				panic!("Error in client writing; err = {:?}", e);
			}
		}
	}

	fn reregister(&self, event_loop: &mut mio::EventLoop<ChatServer>) {
		// change the type of event notifications we want according to connection state
		let event_set = match self.state {
			State::Reading(..) => mio::EventSet::readable(),
			State::Writing(..) => mio::EventSet::writable(),
			_ => return,
		};

		event_loop.reregister(&self.socket, self.token, event_set, mio::PollOpt::oneshot()).unwrap();
	}

	fn is_closed(&self) -> bool {
		match self.state {
			State::Closed => true,
			_ => false,
		}
	}
}

impl State {
	fn try_transition_to_reading(&mut self) {
		if !self.write_buf().has_remaining() {
			self.transition_to_reading();
		}
	}

	fn transition_to_reading(&mut self) {
		let mut buf = mem::replace(self, State::Closed).unwrap_write_buf().into_inner();

		buf.clear();
		*self = State::Reading(buf);
	}

	fn try_transition_to_writing(&mut self, remaining: &mut Vec<Vec<u8>>) {
		match self.read_buf().last() {
			Some(&c) if c == b'\n' => {
				// wrap in scope to avoid borrow checker issues
				{
					let s = str::from_utf8(self.read_buf()).unwrap();
					println!("Read from server: {}", s);
				}

				self.transition_to_writing(remaining);
			}

			_ => {}
		}
	}

	fn transition_to_writing(&mut self, remaining: &mut Vec<Vec<u8>>) {
		if remaining.is_empty() {
			*self = State::Closed;
			return;
		}

		let line = remaining.remove(0);
		*self = State::Writing(Cursor::new(line));
	}

	fn read_buf(&self) -> &[u8] {
		match *self {
			State::Reading(ref buf) => buf,
			_ => panic!("Connection not in reading state!"),
		}
	}

	fn mut_read_buf(&mut self) -> &mut Vec<u8> {
		match *self {
			State::Reading(ref mut buf) => buf,
			_ => panic!("Connection not in reading state!"),
		}
	}

	fn write_buf(&self) ->  &Cursor<Vec<u8>> {
		match *self {
			State::Writing(ref buf) => buf,
			_ => panic!("Connection not in writing state!"),
		}
	}

	fn mut_write_buf(&mut self) ->  &mut Cursor<Vec<u8>> {
		match *self {
			State::Writing(ref mut buf) => buf,
			_ => panic!("Connection not in writing state!"),
		}
	}

	fn unwrap_write_buf(self) -> Cursor<Vec<u8>> {
		match self {
			State::Writing(buf) => buf,
			_ => panic!("Connection not in writing state!"),
		}
	}
}

fn run(address: SocketAddr) {
	// Create new event loop
	let mut event_loop = mio::EventLoop::new().unwrap();

	let mut chat_client = ChatClient::new();

	let socket = match TcpStream::connect(&address) {
		Ok(socket) => socket,
		Err(e) => {
			println!("Failed to create socket. Err = {:?}", e);
		}
	};

	chat_client.connections.insert_with(|token| {
		// register socket with event loop
		event_loop.register_opt(
			&socket,
			token,
			mio::EventSet::writable(),
			mio::PollOpt::edge() | mio::PollOpt::oneshot()
		).unwrap();

		Connection {
			socket: socket,
			token: token,
			state: State::Reading(vec![]),
			remaining: vec![],
		}
	});
}

pub fn main() {
	run("127.0.0.1:8080".parse().unwrap());
}
