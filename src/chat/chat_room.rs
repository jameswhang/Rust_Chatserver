use super::types::*;


pub struct ChatRoom {
    name: String,
    clients: ClientMap,
}

impl ChatRoom {
    pub fn new(name: String) {
        ChatRoom {
            name: name,
            clients: ClientMap::new(),
        }
    }

    pub fn join(&mut self, new_client: &ConnectFourClient) ->
        Result<ActionStatus, ActionStatus> {
        if self.clients.len() == 2 {
            Err(ActionStatus::Failed)
        } else {
            self.clients.insert(new_client.Id, new_client);
            Ok(ActionStatus::OK)
        }
    }

    pub fn remove(&mut self, client_id: Id) {
        self.ClientMap.remove(client_id);
    }
}
