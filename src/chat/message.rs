use super::types::*;

#[derive(Debug, PartialEq)]
pub enum MessageType {
	Join,
	Chat,
	Leave,
    /* 2/24/16 james: i don't know what this is
               let me know what this is about later.
               im hacking through to compile this so commenting out for now
	Confirm(MessageType),
               */

}

#[derive(Debug, PartialEq)]
pub struct Message  {
	date : Time,
	sender : Id,
	receiver : Id,
	message_type : MessageType,
	message : String,
}

impl Message {
	pub fn new(date : Time, sender : Id, receiver : Id, message_type : MessageType, message : String) -> Message {
   	Message {
   		date : date,
   		sender : sender,
   		receiver : receiver,
   		message_type : message_type,
   		message : message
   	}
   }
}
