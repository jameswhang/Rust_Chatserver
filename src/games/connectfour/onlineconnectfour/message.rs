use std::fmt;
use std::str;
use super::super::*;
use self::ConnectFourMType::*;




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectFourMType {
    Join,
    Update,
    Exit,
    Confirm,
    Reject,
}

impl ConnectFourMType {
    fn from_str(s : &str) -> Option<ConnectFourMType> {
        match s {
            "JOIN" => Some(ConnectFourMType::Join),
            "UPDATE" => Some(ConnectFourMType::Update),
            "EXIT" => Some(ConnectFourMType::Exit),
            "CONFIRM" => Some(ConnectFourMType::Confirm),
            "REJECT" => Some(ConnectFourMType::Reject),
            _ => None,
        }
    }

    fn from_string(s : String) -> Option<ConnectFourMType> {
        match &*s {
            "JOIN" => Some(ConnectFourMType::Join),
            "UPDATE" => Some(ConnectFourMType::Update),
            "EXIT" => Some(ConnectFourMType::Exit),
            "CONFIRM" => Some(ConnectFourMType::Confirm),
            "REJECT" => Some(ConnectFourMType::Reject),
            _ => None,
        }
    }

    fn to_string(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for ConnectFourMType {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            match *self {
                ConnectFourMType::Join => "JOIN",
                ConnectFourMType::Update => "UPDATE",
                ConnectFourMType::Exit => "EXIT",
                ConnectFourMType::Reject => "REJECT",
                ConnectFourMType::Confirm =>"CONFIRM"
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
    pub fn new(sender : &String, m_type: ConnectFourMType, content: String) -> ConnectFourMessagePayload {
        ConnectFourMessagePayload {
            sender : sender.clone(),
            m_type : m_type,
            content : content,
        }
    }

    pub fn new_from_str(sender: String, m_type : ConnectFourMType, content : &str) -> ConnectFourMessagePayload {
        ConnectFourMessagePayload {
            sender : sender,
            m_type : m_type,
            content : content.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    pub fn from_string(data : &String) -> Option<ConnectFourMessagePayload> {
        let mut split : Vec<&str> = data.split('|').collect();

        if split.len() != 3 {
            return None;
        }

        let id = split[0];
        let m_type_raw = split[1];
        let content = split[2];

        if let Some(m_type) =  ConnectFourMType::from_string(m_type_raw.to_string()) {
            return Some(ConnectFourMessagePayload::new_from_str(id.to_string(), m_type, content));
        } else {
            return None;
        }
    }

    pub fn from_str(data : &str) ->Option<ConnectFourMessagePayload> {
        ConnectFourMessagePayload::from_string(&data.to_string())
    }

    pub fn m_type(&self) -> ConnectFourMType {
        self.m_type.clone()
    }

    pub fn sender(&self) -> &String {
        &self.sender
    }

    pub fn content(&self) -> &String {
        &self.content
    }
}


impl fmt::Display for ConnectFourMessagePayload {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}|{}",self.sender, self.m_type, self.content)
    }
}


#[cfg(test)]
mod connect_four_message_tests {
    use super::*;
    use super::ConnectFourMType::*;

    #[test]
    fn to_string_test1() {
        assert_eqstr(ConnectFourMessagePayload::new_from_str("Tester".to_string(), Join, "Tester"), "Tester|JOIN|Tester");
    }

    #[test]
    fn to_string_test2() {
        assert_eqstr(ConnectFourMessagePayload::new_from_str("Tester".to_string(), Update, "Tester"), "Tester|UPDATE|Tester");
    }

    #[test]
    fn to_string_test3() {
        assert_eqstr(ConnectFourMessagePayload::new_from_str("Tester".to_string(), Exit, "Tester"), "Tester|EXIT|Tester");
    }

    #[test]
    fn to_string_test4() {
        assert_eqstr(ConnectFourMessagePayload::new_from_str("Tester".to_string(), Confirm, "Tester"), "Tester|CONFIRM|Tester");
    }

    #[test]
    fn to_string_test5() {
        assert_eqstr(ConnectFourMessagePayload::new_from_str("Tester".to_string(), Reject, "Tester"), "Tester|REJECT|Tester");
    }

    fn assert_eqstr(m : ConnectFourMessagePayload, s : &str) {
        assert_eq!(m.to_string(), s.to_string());
    }

    #[test]
    fn from_string_test1() {
        assert_eqstr(ConnectFourMessagePayload::from_str("Tester|JOIN|Tester").unwrap(), "Tester|JOIN|Tester");
    }

    #[test]
    fn from_string_test2() {
        assert_eqstr(ConnectFourMessagePayload::from_str("Tester|CONFIRM|Tester").unwrap(), "Tester|CONFIRM|Tester");
    }

    #[test]
    fn from_string_test3() {
        assert_eqstr(ConnectFourMessagePayload::from_str("Tester|EXIT|Tester").unwrap(), "Tester|EXIT|Tester");
    }

    #[test]
    fn from_string_test4() {
        assert_eqstr(ConnectFourMessagePayload::from_str("Tester|UPDATE|Tester").unwrap(), "Tester|UPDATE|Tester");
    }
    #[test]
    fn from_string_test5() {
        assert_eqstr(ConnectFourMessagePayload::from_str("Tester|REJECT|Tester").unwrap(), "Tester|REJECT|Tester");
    }

    fn get_test_cfmp() -> ConnectFourMessagePayload {
        ConnectFourMessagePayload::new_from_str("Tester".to_string(), Join, "Tester")
    }
}
