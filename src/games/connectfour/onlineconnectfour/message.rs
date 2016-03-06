use ConnectFourMType::*;
use std::fmt;
use super::super::*;



#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectFourMType {
    Join,
    Update,
    Exit,
}

impl ConnectFourMType {
    fn from_string(s : String) -> Option<ConnectFourMType> {
        match &s {
            "JOIN" => Some(ConnectFourMType::Join),
            "UPDATE" => Some(ConnectFourMType::Update),
            "EXIT" => Some(ConnectFourMType::Exit),
            _ => None,
        }
    }

    fn from_u8(data : &[u8]) -> ConnectFourMType {
        ConnectFourMType::from_string(String::from_u8(data)))
    }

    fn to_string(&self) -> String {
        format!("{}", self)
    }

    fn as_bytes(&self) -> &[u8] {
        self.to_string().as_bytes()
    }
}

impl fmt::Display for ConnectFourMType {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            match *self {
                ConnectFourMType::Join => "JOIN",
                ConnectFourMType::Update => "UPDATE",
                ConnectFourMType::Exit => "EXIT",
            }
        )
    }
}



#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ConnectFourMessagePayload {
    sender : String,
    m_type : ConnectFourMType,
    content : String,
}


impl ConnectFourMessagePayload {
    pub fn new(sender : String, m_type: ConnectFourMType, content: String) -> ConnectFourMessagePayload {
        ConnectFourMessagePayload {
            sender : sender,
            m_type : m_type,
            content : content,
        }
    }

    pub fn new_from_str(sender: String, m_type, content : &str) -> ConnectFourMessagePayload {
        ConnectFourMessagePayload {
            sender : sender,
            m_type : m_type,
            content : content.to_string(),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.to_string().as_bytes()
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    pub fn from_string(data : String) -> Option<ConnectFourMessagePayload> {
        let mut split = data.split('|');

        if split.len() != 3 {
            return None;
        }

        let id = split.nth(0).unwrap();
        let m_type_raw = split.nth(1).unwrap();
        let content = split.nth(2).unwrap();

        if let Some(m_type) =  ConnectFourMType::from_string(m_type_raw.to_string()) {
            return Some(
                ConnectFourMessagePayload {
                    sender : id,
                    m_type : m_type,
                    content : content,
            });
        } else {
            return None;
        }
    }

    pub fn from_u8(data: &[u8]) -> Option<ConnectFourMessagePayload> {
        self.from_string(String::from_u8(data))
    }
}


impl fmt::Display for ConnectFourMessagePayload {
    fn fmt(&self, f : &mut Formatter) -> fmt::Result {
        write!(f, "{}|{}|{}",self.sender, self.m_type, self.content)
    }
}
