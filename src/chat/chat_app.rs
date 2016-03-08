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

    pub fn handle_server_message(&mut self, tok : Token, s : String) -> ServerResponse {
        println!("Handling server message: {}", s);
        if let Some(cm) = Message::from_string(&s) {
            let ret =
            match cm.message_type() {
                // Grab an ID from the server
                MessageType::Connect => {
                    println!("Handling connect!");
                    self.handle_connect(cm, tok)
                },

                // Show all the rooms available
                MessageType::Show => {
                    println!("Handling show!");
                    self.handle_show(cm, tok)
                },

                // Join a room.
            	Join =>  {
                    println!("Handling join!");
                    self.handle_join(cm, tok)
                },

                // Leave a room
            	Leave =>  {
                    println!("Handling leave!");
                    self.handle_leave(cm, tok)
                },

                // Action
            	Action => {
                    println!("Handling action!");
                    self.handle_action(cm, tok)
                },

                _ => {
                    //It's either a confirm or reject
                    unimplemented!();
                },
            };

            println!("{}", ret);
            return ret;
        } else {
            unimplemented!();
        }
    }

    fn handle_show(&mut self, cm : Message, tok : Token) -> ServerResponse  {
        unimplemented!();
    }

    fn handle_connect(&mut self, cm : Message, tok : Token) -> ServerResponse {
        let player_id = cm.payload().clone();
        let mid = cm.id().clone();

        if let Vacant(ic_entry) = self.id_to_conn.entry(player_id.clone()) {
            //new connection - new name
            if let Vacant(ci_entry) = self.conn_to_id.entry(tok) {
                ic_entry.insert(tok);
                ci_entry.insert(player_id.clone());

                let mconfirm = Message::new(cm.id().clone(), UTC::now(),
                            "SERVER".to_string(), cm.sender().clone(), Confirm(mid), format!("Welcome {}", player_id));

                ServerResponse::new_with_toks(mconfirm, vec![tok])
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

            return ServerResponse::new_with_toks(mreject, vec![tok]);
        }
    }

    fn handle_join(&mut self, cm : Message, tok : Token) -> ServerResponse {
        let player_id = cm.sender().clone();
        let mid = cm.id().clone();

        if self.verify_id(&player_id, tok) {
            match self.userid_to_room.entry(player_id.clone()) {
                //already part of a room, ask them to leave it
                Occupied(_) => {
                    let mreject = Message::new(cm.id().clone(), UTC::now(),
                                "SERVER".to_string(), cm.sender().clone(), Reject(mid), "Please leave your room first".to_string());
                    ServerResponse::new_with_toks(mreject, vec![tok])
                },

                //user is free to join any room they wnat
                Vacant(good_entry) => {
                    // room exists - go for it
                    if self.id_to_room.contains_key(cm.payload()) {
                        good_entry.insert(cm.payload().clone());
                        let mconfirm = Message::new(cm.id().clone(), UTC::now(),
                                    "SERVER".to_string(), cm.sender().clone(), Confirm(mid), format!("Welcome to: {}", cm.payload()));
                        ServerResponse::new_with_toks(mconfirm, vec![tok])
                    } else {
                        let mreject = Message::new(cm.id().clone(), UTC::now(),
                                    "SERVER".to_string(), cm.sender().clone(), Reject(mid), "No room with that name found".to_string());
                        ServerResponse::new_with_toks(mreject, vec![tok])
                    }
                }
            }
        } else {
            // Shouldn't even be here unless a fake ID was generated somehow
            println!("WARNING: Unverified ID");
            let mreject = Message::new(cm.id().clone(), UTC::now(),
                        "SERVER".to_string(), cm.sender().clone(), Reject(mid),
                        "Unverified ID".to_string());
            ServerResponse::new_with_toks(mreject, vec![tok])
        }
    }

    fn handle_leave(&mut self, cm : Message, tok : Token) -> ServerResponse {
        let player_id = cm.sender().clone();
        let mid = cm.id().clone();

        if self.verify_id(&player_id, tok) {
            match self.id_to_room.entry(player_id) {
                //not in a room to leave from
                Vacant(_) => {
                    let mreject = Message::new(cm.id().clone(), UTC::now(), "SERVER".to_string(),
                        cm.sender().clone(), Reject(mid), "You are not currently in a room".to_string());
                    ServerResponse::new_with_toks(mreject, vec![tok])
                },

                //user is in a room, make it blank
                Occupied(good_entry) => {
                    good_entry.remove();
                    let mconfirm = Message::new(cm.id().clone(), UTC::now(),
                                "SERVER".to_string(), cm.sender().clone(), Confirm(mid), "You've left the room.".to_string());
                    ServerResponse::new_with_toks(mconfirm, vec![tok])
                }
            }
        } else {
            // Shouldn't even be here unless a fake ID was generated somehow
            println!("WARNING: Unverified ID");
            let mreject = Message::new(cm.id().clone(), UTC::now(),
                        "SERVER".to_string(), cm.sender().clone(), Reject(mid),
                        "Unverified ID".to_string());
            ServerResponse::new_with_toks(mreject, vec![tok])
        }
    }



    fn handle_action(&mut self, cm : Message, tok : Token) -> ServerResponse {
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
