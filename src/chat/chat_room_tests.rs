
#[cfg(test)]
mod chat_room_tests {
    use std::net::{TcpStream};

    use super::chat_client::{ChatClient};
    use super::chat_room::{ChatRoom};
    use super::types::{ClientMap};

    fn instantiate_chat_room() -> ChatRoom {
        let map = ClientMap::new();
        let name = "TestChatRoom".to_string();

        ChatRoom {
            name: name,
            map: map,
        }
    }

    #[test]
    fn test_remove_present() {
        unimplemented!();
    }
}
