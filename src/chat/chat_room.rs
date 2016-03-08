extern crate games;

use super::types::*;
use super::message::Message;
use super::message::MessageType::*;
use super::chat_client::{ChatClient};

use self::games::connectfour::ConnectFourServer;
use std::collections::HashSet;
use std::io::{Write};


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ChatRoom {
    name: Id,
    clients: Vec<Id>,
    game : ConnectFourServer,
}

impl ChatRoom {
    pub fn new(name: String) -> ChatRoom {
        ChatRoom {
            name: name,
            clients: vec![],
            game : ConnectFourServer::new(),
        }
    }

    pub fn handle_message(&mut self, cm : &Message) -> (Vec<Id>, String) {
        if cm.message_type() != Action {
            // Shouldn't be here
            unreachable!();
        } else {
            match self.game.handle_message(cm.payload()) {
                Ok(messages) => {
                    let mut result:String = String::new();
                    for ref message in &messages {
                        result = result.clone() + message.to_owned();
                    }
                    (self.clients.clone(), result)
                },

                Err(s) => {
                    unimplemented!();
                },
            }
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
