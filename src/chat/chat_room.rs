use super::types::*;
use super::chat_client::{ChatClient};


pub struct ChatRoom<'a> {
    name: String,
    clients: ClientMap<'a>,
}

impl<'a> ChatRoom<'a> {
    pub fn new(name: String) -> ChatRoom<'a> {
        ChatRoom {
            name: name,
            clients: ClientMap::new(),
        }
    }

    pub fn join(&mut self, new_client: &'a ChatClient) ->
        Result<ActionStatus, ActionStatus> {
            /*
        if self.clients.len() == 2 {
            Err(ActionStatus::Failed)
        } else {
            self.clients.insert(new_client.id, new_client);
            Ok(ActionStatus::OK)
        }
        */
            unimplemented!();
    }

    pub fn remove(&mut self, client_id: &Id) {
        self.clients.remove(client_id);
    }
}
