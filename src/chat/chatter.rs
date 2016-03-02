#[doc="
 Holds the details of a chatter. Isomorphic, meaning has all the information needed for both
 the client side and the server side. Client side connection allows chatter to send message
 into the chatroom while on the server it allows messages to be broadcast to the user
"]

use std::net::TcpStream;
use super::types::*;
use super::messages::*;
use self::ChatterStatus::*;
use self::ActionStatus::*;



#[derive(Debug, PartialEq)]
pub enum ChatterStatus {
	SelectingAction,
	SelectingRoom,
	InRoom(Id),
}


#[derive(Debug, PartialEq)]
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

/*
impl PartialEq for TcpStream {
    fn eq(&self, other: &TcpStream) -> bool {
        if let Some(my_peer) = self.peer_addr() {
            if let Some(other_peer) = other.peer_addr() {
                return my_peer == other_peer;
            }
        }
    }
}
*/


impl Chatter {
	
	pub fn new(username : String, connection : TcpStream) -> Chatter {
		Chatter { 
			username : username as Id,
			connection : connection,
			status : ChatterStatus::SelectingAction,
		}
	}

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
			InRoom(room_id) => {
				self.send_message(MessageType::Chat, &chat_message);
				Ok(OK)
			},

			_ => Err(Invalid)
		}
	}

	pub fn receive_message(&mut self) -> Option<ChatMessage> {
		unimplemented!();
	}
	
	pub fn leave_room(&mut self) -> Result<ActionStatus, ActionStatus> {
		match self.status {
			InRoom(room) => {
				self.send_message(MessageType::Leave, &"".to_string());
				Ok(OK); // not elegant // i agree
			},

			_ => Err(Invalid),
		}
	}
}
