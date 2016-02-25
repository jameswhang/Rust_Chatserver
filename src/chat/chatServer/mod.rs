use super::types::*;
use std::collections::HashMap;
use super::chatter::{Chatter};
use super::ChatRoom::*;


pub struct ChatServer {
	name : String,
	active_users : UserMap, 
	chat_rooms : RoomMap,
}



impl ChatServer {
	pub fn new(name : String) -> ChatServer {
		ChatServer {
            name: name,
			active_users : UserMap::new(),
			chat_rooms : RoomMap::new(),
		}
	}

	pub fn add_chatter(){ unimplemented!();}
	pub fn add_room(){ unimplemented!();}
	pub fn remove_room(){ unimplemented!();}
	pub fn remove_chatter(){ unimplemented!();}
}
