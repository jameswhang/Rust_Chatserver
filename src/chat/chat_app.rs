extern crate mio;
extern crate chrono;

use super::types::*;
use super::message::*;
use super::message::MessageType::*;
use super::chat_room::*;

use self::mio::*;
use self::chrono::*;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChatApp {
    conn_to_id : HashMap<Token, Id>,
    id_to_conn : HashMap<Id, Token>,
    id_to_room : HashMap<Id, ChatRoom>,
    userid_to_room : HashMap<Id, Id>,
}


impl ChatApp {
    pub fn new() -> ChatApp {
        let mut rooms = HashMap::new();
        let roomname = "RustLang".to_string();
        rooms.insert(roomname.clone(), ChatRoom::new(roomname.clone()));

        ChatApp {
            conn_to_id : HashMap::new(),
            id_to_conn : HashMap::new(),
            id_to_room : rooms,
            userid_to_room : HashMap::new(),
        }
    }

    pub fn handle_server_message(&mut self, tok : Token, s : String) -> Vec<ServerResponse> {
        println!("Handling server message: {}", s);
        if let Some(dm) = Message::from_str(s.trim()) {
            let cm = Message::new(dm.id().trim().to_string(), dm.date(), dm.sender().trim().to_string(),
                                    dm.receiver().trim().to_string(), dm.message_type(), dm.payload().trim().to_string());
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

                    let mreject = Message::new(cm.id().clone(), UTC::now(),
                                "SERVER".to_string(), cm.sender().clone(), Reject(cm.id().clone()), "Please leave your room first".to_string());
                    vec![ServerResponse::new_with_toks(mreject, vec![tok])]
                    //It's either a confirm or reject
                },
            };

            println!("{:?}", ret);
            return ret;
        } else {
            unimplemented!();
        }
    }

    fn handle_show(&mut self, cm : Message, tok : Token) -> Vec<ServerResponse>  {
        let player_id = cm.sender().trim().to_string();
        let mid = cm.id().clone();

        if self.verify_id(&player_id, tok) {
            match self.userid_to_room.entry(player_id.clone()) {
                //already part of a room, ask them to leave it
                Occupied(_) => {
                    let mreject = Message::new(cm.id().clone(), UTC::now(),
                                "SERVER".to_string(), cm.sender().clone(), Reject(mid), "Please leave your room first".to_string());
                    vec![ServerResponse::new_with_toks(mreject, vec![tok])]
                },

                //user is free to join any room they wnat
                Vacant(_) => {
                    let room_names  = self.id_to_room.keys().fold("".to_string(), |acc, ref x| format!("{}\n{}", acc, x));
                    println!("room_names{:?}", room_names );
                    let mconfirm = Message::new(cm.id().clone(), UTC::now(),
                                "SERVER".to_string(), cm.sender().clone(), Show, room_names);
                    vec![ServerResponse::new_with_toks(mconfirm, vec![tok])]
                }
            }
        } else {
            // Shouldn't even be here unless a fake ID was generated somehow
            println!("WARNING: Unverified ID");
            let mreject = Message::new(cm.id().clone(), UTC::now(),
                        "SERVER".to_string(), cm.sender().clone(), Reject(mid),
                        "Unverified ID".to_string());
            vec![ServerResponse::new_with_toks(mreject, vec![tok])]
        }
    }

    fn handle_connect(&mut self, cm : Message, tok : Token) -> Vec<ServerResponse> {
        let player_id = cm.payload().trim().to_string();
        let mid = cm.id().clone();

        if let Vacant(ic_entry) = self.id_to_conn.entry(player_id.clone()) {
            //new connection - new name
            if let Vacant(ci_entry) = self.conn_to_id.entry(tok) {
                ic_entry.insert(tok);
                ci_entry.insert(player_id.clone());

                let mconfirm = Message::new(cm.id().clone(), UTC::now(),
                            "SERVER".to_string(), cm.sender().clone(), Confirm(mid), format!("Welcome {:?}", player_id));

                vec![ServerResponse::new_with_toks(mconfirm, vec![tok])]
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

            return vec![ServerResponse::new_with_toks(mreject, vec![tok])];
        }
    }

    fn handle_join(&mut self, cm : Message, tok : Token) -> Vec<ServerResponse> {
        let player_id = cm.sender().trim().to_string();
        let mid = cm.id().clone();

        if self.verify_id(&player_id, tok) {
            match self.userid_to_room.entry(player_id.clone()) {
                //already part of a room, ask them to leave it
                Occupied(_) => {
                    let mreject = Message::new(cm.id().clone(), UTC::now(),
                                "SERVER".to_string(), cm.sender().clone(), Reject(mid), "Please leave your room first".to_string());
                    vec![ServerResponse::new_with_toks(mreject, vec![tok])]
                },

                //user is free to join any room they wnat
                Vacant(good_entry) => {
                    // room exists - go for it
                    // TODO// Add person to chat room
                    let roomname = cm.payload().trim().to_string();

                    //SUCCESS!
                    if let Some(chat_room) = self.id_to_room.get_mut(&roomname) {
                        good_entry.insert(roomname.clone());
                        chat_room.join(&player_id);

                        let mconfirm = Message::new(cm.id().clone(), UTC::now(),
                                    "SERVER".to_string(), cm.sender().clone(), Confirm(mid), format!("Welcome to: {}", roomname.clone()));
                        vec![ServerResponse::new_with_toks(mconfirm, vec![tok])]
                    } else {
                        let mreject = Message::new(cm.id().clone(), UTC::now(),
                                    "SERVER".to_string(), cm.sender().clone(), Reject(mid), "No room with that name found".to_string());
                        vec![ServerResponse::new_with_toks(mreject, vec![tok])]
                    }
                }
            }
        } else {
            // Shouldn't even be here unless a fake ID was generated somehow
            println!("WARNING: Unverified ID");
            let mreject = Message::new(cm.id().clone(), UTC::now(),
                        "SERVER".to_string(), cm.sender().clone(), Reject(mid),
                        "Unverified ID".to_string());
            vec![ServerResponse::new_with_toks(mreject, vec![tok])]
        }
    }

    fn handle_leave(&mut self, cm : Message, tok : Token) -> Vec<ServerResponse> {
        let player_id = cm.sender().trim().to_string();
        let mid = cm.id().clone();

        if self.verify_id(&player_id, tok) {
            match self.userid_to_room.entry(player_id) {
                //not in a room to leave from
                Vacant(_) => {
                    let mreject = Message::new(cm.id().clone(), UTC::now(), "SERVER".to_string(),
                        cm.sender().clone(), Reject(mid), "You are not currently in a room".to_string());
                    vec![ServerResponse::new_with_toks(mreject, vec![tok])]
                },

                //user is in a room, make it blank
                Occupied(good_entry) => {
                    good_entry.remove();
                    let mconfirm = Message::new(cm.id().clone(), UTC::now(),
                                "SERVER".to_string(), cm.sender().clone(), Confirm(mid), "You've left the room.".to_string());
                    vec![ServerResponse::new_with_toks(mconfirm, vec![tok])]
                }
            }
        } else {
            // Shouldn't even be here unless a fake ID was generated somehow
            println!("WARNING: Unverified ID");
            let mreject = Message::new(cm.id().clone(), UTC::now(),
                        "SERVER".to_string(), cm.sender().clone(), Reject(mid),
                        "Unverified ID".to_string());
            vec![ServerResponse::new_with_toks(mreject, vec![tok])]
        }
    }



    fn handle_action(&mut self, cm : Message, tok : Token) -> Vec<ServerResponse> {
        let player_id = cm.sender();
        let mid = cm.id().clone();

        if self.verify_id(&player_id, tok) {
            match self.userid_to_room.get(player_id) {
                //not in a room to leave from
                None => {
                    let mreject = Message::new(cm.id().clone(), UTC::now(), "SERVER".to_string(),
                        cm.sender().clone(), Reject(mid), "You are not currently in a room".to_string());
                    vec![ServerResponse::new_with_toks(mreject, vec![tok])]
                },

                //user is in a room
                Some(chat_room_id) => {
                    if let Some(chat_room) = self.id_to_room.get_mut(chat_room_id) {
                        //pass up the action, receive the response
                        let mut ret = vec![];
                        let mut clients = vec![];

                        for client_id in chat_room.clients() {
                            clients.push(self.id_to_conn.get(client_id).unwrap().clone());
                        }

                        match chat_room.handle_message(cm.payload().clone()) {
                            Ok(responses) => {
                                //send all the responses
                                for response in responses {
                                    let mconfirm = Message::new(cm.id().clone(), UTC::now(),
                                                "SERVER".to_string(), cm.sender().clone(), Confirm(mid.clone()), response);

                                    ret.push(ServerResponse::new_with_toks(mconfirm, clients.clone()));
                                }
                            },

                            Err(s) => {
                                let mreject = Message::new(cm.id().clone(), UTC::now(),
                                            "SERVER".to_string(), cm.sender().clone(), Reject(mid), s.to_string());

                                ret.push(ServerResponse::new_with_toks(mreject, clients.clone()));
                            }
                        }

                        return ret;
                    } else {
                        let mreject = Message::new(cm.id().clone(), UTC::now(), "SERVER".to_string(),
                            cm.sender().clone(), Reject(mid), "Couldn't find your room".to_string());
                        vec![ServerResponse::new_with_toks(mreject, vec![tok])]
                    }
                }
            }
        } else {
            // Shouldn't even be here unless a fake ID was generated somehow
            println!("WARNING: Unverified ID");
            let mreject = Message::new(cm.id().clone(), UTC::now(),
                        "SERVER".to_string(), cm.sender().clone(), Reject(mid),
                        "Unverified ID".to_string());
            vec![ServerResponse::new_with_toks(mreject, vec![tok])]
        }
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
