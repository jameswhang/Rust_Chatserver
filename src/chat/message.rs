extern crate chrono;
extern crate mio;

use std::convert::AsRef;
use super::types::*;
use self::chrono::*;
use self::mio::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MessageType {
    Connect,
    Join,
    Chat,
    Show,
    Leave,
    Action,
    Confirm(Id),
    Reject(Id),
}

/// Uniquely Identifiable Message Struct via message_id, date, sender
/// Design requires that the user will not generate the same id at the same time
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Message  {
    message_id : Id,
    date : DateTime<UTC>,
    sender : Id,
    receiver : Id,
    message_type : MessageType,
    payload: String,
}

impl Message {
    pub fn new(message_id : String, date : Time, sender : String, receiver : String, message_type : MessageType, message : String) -> Message {
        Message {
            message_id : message_id as Id,
            date : date,
            sender : sender as Id,
            receiver : receiver as Id,
            message_type : message_type,
            payload: message
        }
    }

    pub fn from_string(s : String, token: mio::Token) -> Option<Message> {
        match s.as_ref() {
            "CONNECT" => {
                Some(
                    Message {
                        message_id: "".to_string(),
                        date: UTC::now(),
                        sender: "".to_string(),
                        receiver: "SERVER".to_string(),
                        message_type: MessageType::Connect,
                        payload: format!("{}", token.as_usize()),
                    })
            },

            "JOIN" => {
                Some( Message {
                    message_id: "".to_string(),
                    date: UTC::now(),
                    receiver: "SERVER".to_string(),
                    message_type: MessageType::Join,
                    sender: format!("{}", token.as_usize()),
                    payload: "FIXME".to_string(),
                })
            },

            _ => {
                None
            }
        }
    }

    pub fn to_string(&self) -> String {
        unimplemented!();
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    pub fn id(&self) -> &Id {
        &self.message_id
    }

    pub fn date(&self) -> Time {
        self.date.clone()
    }

    pub fn sender(&self) -> &Id {
        &self.sender
    }

    pub fn receiver(&self) -> &Id {
        &self.receiver
    }

    pub fn message_type(&self) -> MessageType {
        self.message_type.clone()
    }

    pub fn payload(&self) -> &String {
        &self.payload
    }
}

pub fn str_to_date(date : &str) -> ParseResult<DateTime<UTC>> {
    let mut time = date.to_string();
    time.truncate(26);
    UTC.datetime_from_str(&*time, "%Y-%m-%d %H:%M:%S.%f")
}

/// Used for passing message between Server I/O and ServerApp
pub struct ServerResponse {
    pub clients: Vec<mio::Token>, // vector of clients to receive the msg
    pub message: Message, // msg to be written to the client
}

impl ServerResponse {
    pub fn new(msg: Message) -> ServerResponse {
        ServerResponse {
            clients: Vec::new(),
            message: msg,
        }
    }

    pub fn add_client(&mut self, token: mio::Token) {
        self.clients.push(token.clone());
    }

    pub fn clients(self) -> Vec<mio::Token> {
        self.clients
    }

    pub fn message(self) -> Message {
        self.message
    }
}
