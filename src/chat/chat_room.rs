extern crate games;

use super::types::*;
use super::message::Message;
use super::message::MessageType::*;
use super::chat_client::{ChatClient};

use self::games::connectfour::{ConnectFourServer};
use self::games::connectfour::onlineconnectfour::ConnectFourMessagePayload;
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

    pub fn handle_message(&mut self, cm : String) -> Result<Vec<String>, &str> {
        // if let Some(gm) = ConnectFourMessagePayload::from_string(&cm) {
            // if gm.m_type() != Action {
            //     // Shouldn't be here
            //     unreachable!();
            // } else {
                self.game.handle_message(&cm)
                // match self.game.handle_message(cm.payload()) {
                //     Ok(messages) => {
                //         Ok(messages)
                //     },
                //
                //     Err(s) => {
                //         Err(vec!["Received in valid game commands".to_string()])
                //     },
                // }
            // }
        // }
    }

    pub fn join(&mut self, new_client: &Id) {
        self.clients.push(new_client.clone());
    }

    pub fn leave(&mut self, client_id: &Id) {
        if let Ok(cindex) = self.clients.binary_search(client_id) {
            self.clients.remove(cindex);
        }
    }

    pub fn clients(&self) -> &Vec<Id> {
        &self.clients
    }
}
