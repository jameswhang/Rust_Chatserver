extern crate chrono;

use std::collections::HashMap;
use super::chatter::{Chatter};
use super::chat_room::*;
use self::chrono::*;


pub type Id = String;
pub type ClientMap = HashMap<Id, &ConnectFourClient>;
pub type RoomMap = HashMap<String, ChatRoom>;
pub type Time = DateTime<UTC>;


/// Used for feedback on
#[derive(Debug, PartialEq)]
pub enum ActionStatus {
	OK,
	Invalid,
	Failed,
}
