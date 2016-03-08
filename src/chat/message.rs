extern crate chrono;
extern crate mio;

use std::convert::AsRef;
use std::fmt;

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

impl MessageType {
    fn from_str(s : &str) -> Option<MessageType> {
        match s {
            "CONNECT" => Some(MessageType::Connect),
            "CHAT" => Some(MessageType::Chat),
            "SHOW" => Some(MessageType::Show),
            "JOIN" => Some(MessageType::Join),
            "ACTION" => Some(MessageType::Action),
            "LEAVE" => Some(MessageType::Leave),
            _ => {
                let splits : Vec<&str> = s.split(":").collect();

                if splits.len() != 2 {
                    return None;
                }

                match splits[0] {
                    "CONFIRM" => Some(MessageType::Confirm(splits[1].to_string())),
                    "REJECT" => Some(MessageType::Reject(splits[1].to_string())),
                    _ => None,
                }
            },
        }
    }

    fn from_string(s : &String) -> Option<MessageType> {
        MessageType::from_str(&*s)
    }

    fn to_string(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let s;

        match *self {
            MessageType::Connect => { s = "CONNECT".to_string() } ,
            MessageType::Join => { s = "JOIN".to_string()},
            MessageType::Action => { s = "ACTION".to_string() } ,
            MessageType::Chat => { s = "CHAT".to_string() },
            MessageType::Show => { s = "SHOW".to_string() },
            MessageType::Leave => { s = "LEAVE".to_string() },
            MessageType::Reject(ref id) =>  { s = "REJECT:".to_string() + &*id },
            MessageType::Confirm(ref id) => { s = "CONFIRM:".to_string() + &*id },
        }

        write!(f, "{}", s)
    }
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

    pub fn from_string(s : &String) -> Option<Message> {
        let mut split : Vec<&str> = s.split('=').collect();

        if split.len() != 6 {
            return None;
        }

        let mid = split[0];
        let date = split[1];
        let sender = split[2];
        let receiver = split[3];
        let m_type_raw = split[4];
        let payload = split[5];

        if let Some(m_type) =  MessageType::from_str(m_type_raw) {
            if let Ok(utcdate) = str_to_date(date) {
                return Some(Message::new(mid.trim().to_string(), utcdate, sender.trim().to_string(), receiver.trim().to_string(), m_type, payload.trim().to_string()));
            }
        }

        return None;
    }

    pub fn from_str(s : &str) -> Option<Message> {
        Message::from_string(&s.to_string())
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    pub fn id(&self) -> &Id {
        &self.message_id
    }

    pub fn date(&self) -> DateTime<UTC> {
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

impl fmt::Display for Message {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}={}={}={}={}={}", self.message_id, self.date, self.sender, self.receiver, self.message_type, self.payload)
    }
}

/// Used for passing message between Server I/O and ServerApp
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ServerResponse {
    pub clients: Vec<mio::Token>, // vector of clients to receive the msg
    pub message: String, // msg to be written to the client
}

impl ServerResponse {
    pub fn new(msg: Message) -> ServerResponse {
        ServerResponse {
            clients: Vec::new(),
            message: msg.to_string(),
        }
    }
    pub fn new_with_toks(msg: Message, toks : Vec<Token>) -> ServerResponse {
        ServerResponse {
            clients: toks,
            message: msg.to_string(),
        }
    }

    pub fn add_client(&mut self, token: mio::Token) {
        self.clients.push(token.clone());
    }

    pub fn clients(&self) -> Vec<mio::Token> {
        self.clients.clone()
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl fmt::Display for ServerResponse {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "clients: {:?}\nmessage: {}", self.clients, self.message)
    }
}
