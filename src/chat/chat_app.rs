extern crate mio;
extern crate chrono;

use super::types::*;
use super::message::*;
use super::message::MessageType::*;
//NEED TO FIX THIS
type ChatRoom = String;
// use super::chat_room::*;

use self::mio::*;
use self::mio::tcp::*;
use self::mio::util::Slab;
use self::chrono::*;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{self, Occupied, Vacant};
use std::io::{Write};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChatApp {
    conn_to_id : HashMap<Token, Id>,
    id_to_conn : HashMap<Id, Token>,
    id_to_room : HashMap<Id, ChatRoom>,
    userid_to_room : HashMap<Id, Id>,
}


impl ChatApp {
    pub fn new() -> ChatApp {
        ChatApp {
            conn_to_id : HashMap::new(),
            id_to_conn : HashMap::new(),
            id_to_room : HashMap::new(),
            userid_to_room : HashMap::new(),
        }
    }

    pub fn handle_server_message(&mut self, tok : Token, s : String) {
        if let Some(cm) = Message::from_string(s) {
            match cm.message_type() {
                Connect => { self.handle_connect(cm, tok); },

            	Join =>   { self.handle_join(cm, tok); }

            	Leave =>  { self.handle_leave(cm, tok); },

            	Action => { self.handle_action(cm, tok); },

                _ => {
                    //It's either a confirm or reject
                    unimplemented!();
                },
            }
        } else {
            unimplemented!();
        }
    }

    fn handle_connect(&mut self, cm : Message, tok : Token) {
        let player_id = cm.message().clone();
        let mid = cm.id().clone();

        if let Vacant(ic_entry) = self.id_to_conn.entry(player_id.clone()) {
            //new connection - new name
            if let Vacant(ci_entry) = self.conn_to_id.entry(tok) {
                ic_entry.insert(tok);
                ci_entry.insert(player_id);

                let mconfirm = Message::new(cm.id().clone(), UTC::now(),
                            "SERVER".to_string(), cm.sender().clone(), Confirm(mid), "".to_string());
                
                // TODO: Return something
            }

            //old connection wants new name
            else {
                //could handle this more elegantly, but very unlikely unless spoofing packets
                unreachable!();
            }
        }

        // id already taken, refuse "connection"
        else {
            let mreject = Message::new(cm.id().clone(), UTC::now(),
                        "SERVER".to_string(), cm.sender().clone(), Reject(mid), "Requested ID is already taken".to_string());

            //self.handle_to_mio_to_write_with.write_to_client_stream(mreject.into_bytes().as_slice());
            // TODO: Return something
        }
    }

    fn handle_join(&mut self, cm : Message, tok : Token) {
        let player_id = cm.sender().clone();
        let mid = cm.id().clone();

        if self.verify_id(&player_id, tok) {
            match self.userid_to_room.entry(player_id.clone()) {
                //already part of a room, ask them to leave it
                Occupied(_) => {
                    let mreject = Message::new(cm.id().clone(), UTC::now(),
                                "SERVER".to_string(), cm.sender().clone(), Reject(mid), "Please leave your room first".to_string());
//                    self.handle_to_mio_to_write_with.write(mreject.into_bytes().as_slice());
//                    // TODO: Return something
                },

                //user is free to join any room they wnat
                Vacant(good_entry) => {
                    // room exists - go for it
                    if self.id_to_room.contains_key(cm.message()) {
                        good_entry.insert(cm.message().clone());
                        let mconfirm = Message::new(cm.id().clone(), UTC::now(),
                                    "SERVER".to_string(), cm.sender().clone(), Confirm(mid), format!("Welcome to: {}", cm.message()));
                        //self.handle_to_mio_to_write_with.write(mconfirm.into_bytes().as_slice());
                        // TODO: Return something
                    } else {
                        let mreject = Message::new(cm.id().clone(), UTC::now(),
                                    "SERVER".to_string(), cm.sender().clone(), Reject(mid), "No room with that name found".to_string());
                        //self.handle_to_mio_to_write_with.write(mreject.into_bytes().as_slice());
                        //TODO: Return something
                    }
                }
            }
        }
    }

    fn handle_leave(&mut self, cm : Message, tok : Token) {
        let player_id = cm.sender().clone();
        let mid = cm.id().clone();

        if self.verify_id(&player_id, tok) {
            match self.id_to_room.entry(player_id) {
                //not in a room to leave from
                Vacant(_) => {
                    let mreject = Message::new(cm.id().clone(), UTC::now(), "SERVER".to_string(),
                        cm.sender().clone(), Reject(mid), "You are not currently in a room".to_string());
//                    self.handle_to_mio_to_write_with.write(mreject.into_bytes().as_slice());
//                    // TODO: Return something
                },

                //user is in a room, make it blank
                Occupied(good_entry) => {
                    good_entry.remove();
                    let mconfirm = Message::new(cm.id().clone(), UTC::now(),
                                "SERVER".to_string(), cm.sender().clone(), Confirm(mid), "You've left the room.".to_string());
                    //self.handle_to_mio_to_write_with.write(mconfirm.into_bytes().as_slice());
                    // TODO: Return something
                }
            }
        }
    }



    fn handle_action(&mut self, cm : Message, tok : Token) {
        unimplemented!()
    }


    //checks if the connection is trying to spoof their id
    fn verify_id(&self, player_id : &String, tok : Token) -> bool {
        if let Some(stored_player_id) = self.conn_to_id.get(&tok) {
            return *stored_player_id == *player_id;
        }

        false
    }

    pub fn handle_disconnect(&mut self) {
        unimplemented!();
    }
}
