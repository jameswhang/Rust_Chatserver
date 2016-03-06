use std::fmt;
use super::super::*;



const SERVER_ID : String = "SERVER".to_string();

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ConnectFourServer<T> {
    game: ConnectFour,
    move_history: Vec<ConnectFourMessagePayload>
}


impl ConnectFourServer {
    pub fn new() -> ConnectFourServer<T> {
        ConnectFourServer {
            game : ConnectFour::new(),
            move_history : vec![],
        }
    }

    pub fn handle_message(&mut self, message : String) -> GResult<&str> {
        if let Some(payload) = ConnectFourMessagePayload::from_string(message) {
            match payload::m_type {
                Join => self.handle_join(&payload),
                Update => self.handle_update(&payload),
                Exit => self.handle_exit(&payload),
            }
        } else {
            Err("Invalid payload received")
        }
    }

    fn handle_join(&self, message : &ConnectFourMessagePayload) -> &[ConnectFourMessagePayload] {
        let ret = vec![];

        match self.game.add_player(message.content) {
            Ok(state) => {
                ret.push(ConnectFourMessagePayload::new(SERVER_ID, Join, message.content));
                if self.game.is_full() {
                    ret.push(ConnectFourMessagePayload::new(SERVER_ID, Update, GameState::Ongoing.to_string()));
                }
            },

            Err(s) => {
                ret.push(ConnectFourMessagePayload::new(SERVER_ID, Update, format!("{} failed to join", message.content)))
            }
        }

        ret.as_slice()
    }

    fn handle_update(&self, message : &ConnectFourMessagePayload) -> ConnectFourMessagePayload {
        let player_id = message.sender;

        if !self.game.is_playing(player_id) {
            return ConnectFourMessagePayload::new_from_str(player_id, Update, "You are not a player in this game");
        } else if self.whos_turn() != player_id {
            return ConnectFourMessagePayload::new_from_str(player_id, Update, "It is not your turn yet");
        }

        if let Ok(col) = message.content.parse::<usize>() {
            match self.game.make_move(col) {
                Ok(state) => ConnectFourMessagePayload::new(SERVER_ID, Update, message.content),
                Err(s) => ConnectFourMessagePayload::new_from_str(player_id, Update, "Invalid column attempted. Try again"),
            }
        } else {
            ConnectFourMessagePayload::new_fromt_str(player_id, Update, "Invalid column attempted. Try again")
        }
    }

    fn handle_exit(&self, message : &ConnectFourMessagePayload) -> ConnectFourMessagePayload {
        let player_id = message.sender;


        if !self.game.is_playing(player_id) {
            return ConnectFourMessagePayload::new_from_str(player_id, Update, "You are not a player in this game");
        }

        if let Ok(col) = message.content.parse::<usize>() {
            match self.game.make_move(col) {
                Ok(state) => ConnectFourMessagePayload::new(SERVER_ID, Update, message.content),
                Err(s) => ConnectFourMessagePayload::new_from_str(player_id, Update, "Invalid column attempted. Try again"),
            }
        } else {
            ConnectFourMessagePayload::new_fromt_str(player_id, Update, "Invalid column attempted. Try again")
        }
    }

    fn handle_disconnect(&self, player_id : Id) -> String {
        
    }
}
