use super::types::*;
use super::message::Message;
use super::message::MessageType::*;
use super::chat_client::{ChatClient};
//use super::super::games::connectfour::{ConnectFourServer};
use std::collections::HashSet;
use std::io::{Write};


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ChatRoom<T> {
    name: Id,
    clients: Vec<Id>,
    //game : ConnectFourServer,
    handle : T,
}

impl<T : Write> ChatRoom<T> {
    pub fn new(name: String, handle : T) -> ChatRoom<T> {
        ChatRoom {
            name: name,
            clients: vec![],
            //game : ConnectFourServer::new(),
            handle : handle,
        }
    }

    pub fn handle_message(&mut self, cm : &Message) {
        if cm.message_type() != Action {

        } else {
            /*
            match self.game.handle_message(cm.message()) {
                Ok(messages) => {
                    for ref client in &self.clients {
                        for ref message in &messages {
                            // self.handle.write(&*message.clone().into_bytes().as_slice());
                        }
                    }
                },

                Err(s) => {
                    unimplemented!();
                },
            }
            */
            unimplemented!();
        }
    }

    pub fn join(&mut self, new_client: Id) {
        self.clients.push(new_client);
    }

    pub fn leave(&mut self, client_id: &Id) {
        if let Ok(cindex) = self.clients.binary_search(client_id) {
            self.clients.remove(cindex);
        }
    }
}
