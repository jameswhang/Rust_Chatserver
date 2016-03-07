use std::fmt;
use super::message::*;
use super::message::ConnectFourMType::*;
use super::super::{ConnectFour, Game, TurnBasedGame};


const SERVER_ID : &'static str= "SERVER";

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum ClientState {
    Joining,
    Playing,
    Leaving,
}


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ConnectFourClient {
    game : ConnectFour,
    state : ClientState,
    id : String,
}


impl ConnectFourServer {
    pub fn new() -> ConnectFourServer {
        ConnectFourServer {
            game : ConnectFour::new(),
            move_history : vec![],
        }
    }

    pub fn handle_message(&mut self, message : &String) -> Result<Vec<String>, &str> {
        if let Some(payload) = ConnectFourMessagePayload::from_string(message) {
            match payload.m_type() {
                Join =>   Ok(self.handle_join(&payload),
                Update => Ok(self.handle_update(&payload),
                Exit =>   Ok(self.handle_exit(&payload),
                _ => unimplemented!(),
            }
        } else {
            Err("Invalid payload received")
        }
    }

    pub fn handle_input(&mut self, message : &String) -> Result<Vec<String>>, &str> {
        match *message.trim()to_lowercase() {
            "join"  => {
                self.state = ClientState::Joining;
                self.make_join()
            },

            "leave" => {
                self.state = ClientState::Leaving;
                self.make_leave()
            },

            _ => {
                self.make_update(&message)
            }
        }
    }

    fn make_join(&self) -> Vec<String> {
        vec![ConnectFourMessagePayload::new(&self.id, Join, self.id.clone())]
    }

    fn make_leave(&self) -> Vec<String> {
        vec![ConnectFourMessagePayload::new(&self.id, Exit, self.id.clone())]
    }

    fn make_update(&self, s : String) -> Vec<String> {
        vec![ConnectFourMessagePayload::new(&self.id, Exit, s.clone())]
    }

    fn handle_join(&mut self, message : &ConnectFourMessagePayload) -> Vec<ConnectFourMessagePayload> {
        let _SERVER_ID = SERVER_ID.to_string();
        let mcontent = message.content().clone();
        let player_id = message.sender().clone();


        if player_id == _SERVER_ID {
            if self.id == message.sender() {
                self.state = ClientState::Playing;
            }

            self.game.add_player(mcontent);
        }
    }

    fn handle_update(&mut self, message : &ConnectFourMessagePayload) {
        let _SERVER_ID = SERVER_ID.to_string();
        let player_id = message.sender().clone();

        if player_id == _SERVER_ID {
            if let Ok(col) = message.content().parse::<usize>() {
                self.game.make_move(col);
            }
        }
    }

    pub fn handle_exit(&mut self, message : &ConnectFourMessagePayload) -> Vec<ConnectFourMessagePayload> {
        let _SERVER_ID = SERVER_ID.to_string();
        let player_id = message.sender().clone();

        if player_id == _SERVER_ID {
            self.game.remove_player(&player_id)
        }
    }

    pub fn game(&self) -> &ConnectFour {
        &self.game
    }

    pub fn state(&self) -> &ClientState {
        self.state.clone()
    }
}
