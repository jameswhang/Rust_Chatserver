extern crate games;

use super::types::*;
use self::games::connectfour::{ConnectFourServer};

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
        self.game.handle_message(&cm)
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
