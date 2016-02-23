#[doc="
 Holds the details of a chatter. Isomorphic, meaning has all the information needed for both
 the client side and the server side. Client side connection allows chatter to send message
 into the chatroom while on the server it allows messages to be broadcast to the user
"]


use super::types::*;
use super::messages::*;

pub enum ChatterStatus {
	SelectingAction,
	SelectingRoom,
	InRoom(Id),
}


pub enum ActionStatus {
	OK,
	Invalid,
	Failed,
}


#[derive(Debug, PartialEq)]
pub struct Chatter {
	username : Id,
	connection : TcpStream,
	status : ChatterStatus, 
}


impl Chatter { 
	use ChatterStatus::*;
	use ActionStatus::*;

	pub fn new(username : String, connection : TcpStream) -> Chatter {
		Chatter { 
			username : username as Id,
			connection : connection,
			status : ChatterStatus::SelectingAction,
		}
	}

	pub fn send_message(&mut self, message_type : MessageType, message : &String) -> Result<_> {
		unimplemented!();
	}

	pub fn join_room(&mut self, room_id : Id) -> Result<ActionStatus> {
		match self.status {
			SelectingRoom => {
				self.send_message(MessageType::Join, &(room_id as String));
				Ok(OK)
			},

			_ = > Err(Invalid),
		}
	}
	
	//technically have to match on the results of send messge
	pub fn send_chat(&mut self, chat_message : String) -> Result<ActionStatus> {
		match self.status {
			InRoom(room_id) => {
				self.send_message(Chat, &chat_message);
				Ok(OK)
			},

			_ => Err(Invalid)
		}
	}

	pub fn receive_message(&mut self) -> Option<ChatMessage> {
		unimplemented!();
	}
	
	pub fn leave_room(&mut self) -> Result<ActionStatus> {
		match self.status {
			InRoom(room) => {
				self.send_message(MessageType::Leave, "".to_string());
				Ok(OK); // not elegant
			},

			_ => Err(Invalid),
		}
	}
}
