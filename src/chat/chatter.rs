#[doc="
 Holds the details of a chatter. Isomorphic, meaning has all the information needed for both
 the client side and the server side. Client side connection allows chatter to send message
 into the chatroom while on the server it allows messages to be broadcast to the user
"]

use std::net::TcpStream;
use super::types::*;
use super::message::*;

// used for convenience
use self::ChatterStatus::*;
use super::types::ActionStatus::*;


/// Used to record the current state of the chatter
#[derive(Debug, PartialEq)]
pub enum ChatterStatus {
	SelectingAction,
	SelectingRoom,
    LeavingRoom,
	InRoom,
}

/// Struct used to pass messages. Wraps interaction with TcpStream
#[derive(Debug)]
pub struct Chatter {
	username: Id,
	connection: TcpStream,
	status: ChatterStatus,
}

impl Chatter {
    /// Assumes that a connection already exists and is given a copy by move
    ///
    /// @param username : String
    /// @param connection : TcpStream
    /// @return Chatter
	pub fn new(username : String, connection : TcpStream) -> Chatter {
		Chatter {
			username : username as Id,
			connection : connection,
			status : ChatterStatus::SelectingAction,
		}
	}

    /// Helper function to send messages to connection
    ///
    /// @param message_type : MessageType
    /// @param message : &String
	pub fn send_message(&mut self, message_type : MessageType, message : &String) ->
        Result<ActionStatus, ActionStatus> {
		unimplemented!();
	}

	pub fn join_room(&mut self, room_id : Id) -> Result<ActionStatus, ActionStatus> {
		match self.status {
			SelectingRoom => {
				self.send_message(MessageType::Join, &(room_id as String));
				Ok(OK)
			},

			_ => Err(Invalid),
		}
	}

	//technically have to match on the results of send messge
	pub fn send_chat(&mut self, chat_message : String) -> Result<ActionStatus, ActionStatus> {
		match self.status {
			InRoom => {
				self.send_message(MessageType::Chat, &chat_message);
				Ok(OK)
			},

			_ => Err(Invalid)
		}
	}

	pub fn receive_message(&mut self) -> Option<Message> {
		unimplemented!();
	}

	pub fn leave_room(&mut self) -> Result<ActionStatus, ActionStatus> {
		match self.status {
			InRoom => {
				self.send_message(MessageType::Leave, &"".to_string());
				Ok(ActionStatus::OK) // not elegant // i agree
			},

			_ => Err(Invalid),
		}
	}
}



#[cfg(test)]
mod chatter_tests {

}
