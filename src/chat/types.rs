extern crate chrono;

use std::collections::HashMap;
use super::chatter::{Chatter};
use super::chat_room::*;
use self::chrono::*;


pub type Id = String;
pub type UserMap = HashMap<Id, Chatter>;
pub type RoomMap = HashMap<Id, ChatRoom>;
pub type Time = DateTime<UTC>;
