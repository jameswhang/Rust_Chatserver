extern crate chrono;
extern crate mio;

#[cfg(test)]
mod message_struct_tests {
    use chat::message_tests::chrono::*;
    use chat::message::{Message, MessageType};

    use chat::types::{Id};

    pub fn instantiate_message() -> Message {
        let message = Message::new(
            ("message_id".to_string() as Id),
            UTC::now(),
            ("sender_id".to_string() as Id),
            ("receiver_id".to_string() as Id),
            MessageType::Join,
            "TestMessageString".to_string()
        );
        message
    }

    #[test]
    fn test_id() {
        let message = instantiate_message();
        assert_eq!(message.id(), &("message_id".to_string() as Id));
    }

    #[test]
    fn test_date() {
        let time = UTC::now();
        let message = Message::new(
            ("message_id".to_string() as Id),
            time,
            ("sender_id".to_string() as Id),
            ("receiver_id".to_string() as Id),
            MessageType::Join,
            "TestMessageString".to_string()
        );
        assert_eq!(message.date(), time);
    }

    #[test]
    fn test_sender() {
        let message = instantiate_message();
        assert_eq!(message.sender(), &("sender_id".to_string() as Id));
    }

    #[test]
    fn test_receiver() {
        let message = instantiate_message();
        assert_eq!(message.receiver(), &("receiver_id".to_string() as Id));
    }

    #[test]
    fn test_message_type() {
        let message = instantiate_message();
        assert_eq!(message.message_type(), MessageType::Join);
    }
}

#[cfg(test)]
mod server_response_tests {
    use chat::message::{Message, ServerResponse};
    use chat::message_tests::message_struct_tests::{instantiate_message};
    use chat::message_tests::mio::{Token};

    fn instantiate_server_response() -> ServerResponse {
        let token_list = vec![
            Token((1 as usize)),
            Token((2 as usize)),
            Token((3 as usize)),
            Token((4 as usize)),
            Token((5 as usize)),
        ];
        let new_message = instantiate_message();

        ServerResponse {
            clients: token_list,
            message: new_message,
        }
    }

    #[test]
    fn test_clients() {
        let mut server_response = instantiate_server_response();
        assert_eq!(server_response.clients().len(), 5);
    }

    #[test]
    fn test_message() {
        let new_message = instantiate_message();
        let token_list = vec![Token(1 as usize)];
        let server_response = ServerResponse {
            clients: token_list,
            message: new_message.clone(),
        };

        assert_eq!(server_response.message(), new_message);
    }

    #[test]
    fn test_add_client() {
        let mut server_response = instantiate_server_response();
        let new_token = Token(6 as usize);
        server_response.add_client(new_token.clone());

        let clients = server_response.clients();
        assert_eq!(clients.len(), 6);
        assert_eq!(clients[5], new_token);
    }
}
