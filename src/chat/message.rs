use super::types::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MessageType {
	Join,
	Chat,
	Leave,
	Action ,
	Confirm(Id),
	//used to confirm Message actions
}

/// Uniquely Identifiable Message Struct via message_id, date, sender
/// Design requires that the user will not generate the same id at the same time
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Message  {
	message_id : Id,
	date : Time,
	sender : Id,
	receiver : Id,
	message_type : MessageType,
	message : String,
}

impl Message {
	pub fn new(message_id : String, date : Time, sender : String, receiver : String, message_type : MessageType, message : String) -> Message {
   	Message {
		message_id : message_id as Id,
   		date : date,
   		sender : sender as Id,
   		receiver : receiver as Id,
   		message_type : message_type,
   		message : message
   	}
   }

   fn id(&self) -> &Id {
	   &self.message_id
   }

   fn date(&self) -> Time {
	  self.date.clone()
   }

   fn sender(&self) -> &Id {
	   &self.sender
   }

   fn receiver(&self) -> &Id {
	   &self.receiver
   }

   fn message_type(&self) -> MessageType {
	   self.message_type.clone()
   }

   fn message(&self) -> &String {
	   &self.message
   }
}
