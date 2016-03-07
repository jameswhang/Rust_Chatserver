use std::fmt;
use super::message::*;
use super::message::ConnectFourMType::*;
use super::super::*;




const SERVER_ID : &'static str= "SERVER";

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ConnectFourServer {
    game: ConnectFour,
    move_history: Vec<ConnectFourMessagePayload>
}


impl ConnectFourServer {
    pub fn new() -> ConnectFourServer {
        ConnectFourServer {
            game : ConnectFour::new(),
            move_history : vec![],
        }
    }

    pub fn handle_message(&mut self, message : String) -> Result<Vec<String>, &str> {
        if let Some(payload) = ConnectFourMessagePayload::from_string(message) {
            match payload.m_type() {
                Join =>   Ok(self.handle_join(&payload).into_iter().map(|m| m.to_string()).collect::<Vec<String>>()),
                Update => Ok(self.handle_update(&payload).into_iter().map(|m| m.to_string()).collect::<Vec<String>>()),
                Exit =>   Ok(self.handle_exit(&payload).into_iter().map(|m| m.to_string()).collect::<Vec<String>>()),
            }
        } else {
            Err("Invalid payload received")
        }
    }

    fn handle_join(&mut self, message : &ConnectFourMessagePayload) -> Vec<ConnectFourMessagePayload> {
        let _SERVER_ID = SERVER_ID.to_string();
        let mcontent = message.content().clone();
        let mut ret = vec![];

        match self.game.add_player(message.content()) {
            Ok(state) => {
                ret.push(ConnectFourMessagePayload::new(_SERVER_ID.clone(), Join, mcontent));
                ret.push(ConnectFourMessagePayload::new(_SERVER_ID.clone(), Update, state.to_string()));
            },
            Err(s) => {
                ret.push(ConnectFourMessagePayload::new(_SERVER_ID, Update, format!("{} failed to join", mcontent)))
            }
        }

        ret
    }

    fn handle_update(&mut self, message : &ConnectFourMessagePayload) -> Vec<ConnectFourMessagePayload> {
        let _SERVER_ID = SERVER_ID.to_string();
        let player_id = message.sender().clone();
        let mcontent = message.content().clone();
        let mut ret = vec![];

        if !self.game.is_playing(&player_id) {
            ret.push(ConnectFourMessagePayload::new_from_str(player_id, Update, "You are not a player in this game"));
        } else if *self.game.whos_turn().id() != player_id {
            ret.push(ConnectFourMessagePayload::new_from_str(player_id, Update, "It is not your turn yet"));
        } else {
            if let Ok(col) = message.content().parse::<usize>() {
                match self.game.make_move(col) {
                    Ok(state) => { ret.push(ConnectFourMessagePayload::new(_SERVER_ID, Update, mcontent)); },
                    Err(s) => { ret.push(ConnectFourMessagePayload::new_from_str(player_id, Update, "Invalid column attempted. Try again")); },
                }
            } else {
                ret.push(ConnectFourMessagePayload::new_from_str(player_id, Update, "Invalid column attempted. Try again"));
            }
        }

        ret
    }

    fn handle_exit(&mut self, message : &ConnectFourMessagePayload) -> Vec<ConnectFourMessagePayload> {
        unimplemented!();
        // let _SERVER_ID = SERVER_ID.to_string();
        // let player_id = message.sender().clone();
        // let mut ret = vec![];
        //
        // match self.game.remove_player(&player_id) {
        //     Ok(state) => {
        //
        //     },
        //
        //     Err(s) => {
        //
        //     },
        // }
        // if !self.game.is_playing(&player_id) {
        //     ret.push(ConnectFourMessagePayload::new_from_str(player_id, Update, "You are not a player in this game"));
        // } else {
        //
        //     if let Ok(col) = message.content().parse::<usize>() {
        //         match self.game.make_move(col) {
        //             Ok(state) => { ret.push(ConnectFourMessagePayload::new(_SERVER_ID, Update, mcontent)); },
        //             Err(s) => { ret.push(ConnectFourMessagePayload::new_from_str(player_id, Update, "Invalid column attempted. Try again")); },
        //         }
        //     } else {
        //         ret.push(ConnectFourMessagePayload::new_from_str(player_id, Update, "Invalid column attempted. Try again"));
        //     }
        // }
        //
        // ret
    }

    fn handle_disconnect(&self, player_id : Id) -> Vec<ConnectFourMessagePayload> {
        unimplemented!();
    }
}
