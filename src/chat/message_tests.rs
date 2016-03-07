extern crate chrono;

#[cfg(test)]
mod message_struct_tests {
    use chat::message_tests::chrono::*;
    use chat::message::{Message, MessageType};

    use chat::types::{Id};

    fn instantiate_message() -> Message {
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

    #[test]
    fn test_message() {
        let message = instantiate_message();
        assert_eq!(message.message(), &("TestMessageString".to_string() as Id));
    }
}
