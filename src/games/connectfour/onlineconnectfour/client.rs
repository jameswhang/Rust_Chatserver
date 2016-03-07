use std::fmt;
use super::message::*;
use super::message::ConnectFourMType::*;
use super::super::{ConnectFour, Game, TurnBasedGame};


const SERVER_ID : &'static str= "SERVER";

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ClientState {
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


impl ConnectFourClient {
    pub fn new(id : &String) -> ConnectFourClient {
        ConnectFourClient{
            game : ConnectFour::new(),
            state : ClientState::Joining,
            id : id.clone(),
        }
    }

    pub fn handle_message(&mut self, message : &String) {
        if let Some(payload) = ConnectFourMessagePayload::from_string(message) {
            match payload.m_type() {
                Join =>   self.handle_join(&payload),
                Update => self.handle_update(&payload),
                Exit =>   self.handle_exit(&payload),
                _ => {;},
            }
        }
    }

    pub fn handle_input(&mut self, message : &String) -> Result<Vec<String>, &str> {
        match &*message.trim().to_lowercase() {
            "join"  => {
                self.state = ClientState::Joining;
                return Ok(self.make_join())
            },

            "leave" => {
                self.state = ClientState::Leaving;
                return Ok(self.make_leave())
            },

            _ => {
                return Ok(self.make_update(message))
            }
        }
    }

    fn make_join(&self) -> Vec<String> {
        vec![ConnectFourMessagePayload::new(&self.id, Join, self.id.clone()).to_string()]
    }

    fn make_leave(&self) -> Vec<String> {
        vec![ConnectFourMessagePayload::new(&self.id, Exit, self.id.clone()).to_string()]
    }

    fn make_update(&self, s : &String) -> Vec<String> {
        vec![ConnectFourMessagePayload::new(&self.id, Exit, s.clone()).to_string()]
    }

    fn handle_join(&mut self, message : &ConnectFourMessagePayload){
        let _SERVER_ID = SERVER_ID.to_string();
        let player_id = message.sender().clone();


        if player_id == _SERVER_ID {
            if self.id == *message.sender() {
                self.state = ClientState::Playing;
            }

            self.game.add_player(message.sender());
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

    pub fn handle_exit(&mut self, message : &ConnectFourMessagePayload) {
        let _SERVER_ID = SERVER_ID.to_string();
        let player_id = message.sender().clone();

        if player_id == _SERVER_ID {
            self.game.remove_player(&player_id);
        }
    }

    pub fn game(&self) -> &ConnectFour {
        &self.game
    }

    pub fn state(&self) -> ClientState {
        self.state.clone()
    }
}
