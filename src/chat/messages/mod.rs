use super::types::*;

pub enum MessageType {
	Join,
	Chat,
	Leave,
	Confirm(MessageType),
}

#[derive(Debug, PartialEq)]
pub struct ChatMessage  {
	date : Time,
	sender : Id,
	receiver : Id,
	message_type : MessageType,
	message : String,
}

impl ChatMessage {
	pub fn new(date : Time, sender : Id, receiver : Id, message_type : MessageType, message : String) -> ChatMessage {
   	ChatMessage {
   		date : date,
   		sender : sender, 
   		receiver : receiver,
   		message_type : message_type,
   		message : message
   	}
   }
}
