extern crate chrono;

use std::collections::HashMap;
use super::chat_client::{ChatClient};
use self::chrono::*;


pub type Id = String;
pub type ClientMap<'a> = HashMap<Id, &'a ChatClient>;
pub type Time = DateTime<UTC>;


/// Used for feedback on
#[derive(Debug, PartialEq)]
pub enum ActionStatus {
	OK,
	Invalid,
	Failed,
}
