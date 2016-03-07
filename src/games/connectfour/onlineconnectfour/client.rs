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

            self.game.add_player(message.content());
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
            self.game.remove_player(&message.content());
        }
    }

    pub fn game(&self) -> &ConnectFour {
        &self.game
    }

    pub fn state(&self) -> ClientState {
        self.state.clone()
    }
}


#[cfg(test)]
mod connectfourclient_tests {
    use super::*;
    use super::ClientState::*;
    use super::super::message::*;
    use super::super::super::*;
    use super::super::message::ConnectFourMType::*;

    #[test]
    fn test_joins_game() {
        let mut game = ConnectFourClient::new(&"tester1".to_string());
        let expected = ConnectFour::new_with_players(&"tester1".to_string(), &"tester2".to_string());

        game.handle_message(&ConnectFourMessagePayload::new_from_str("SERVER".to_string(), Join, "tester1").to_string());
        game.handle_message(&ConnectFourMessagePayload::new_from_str("SERVER".to_string(), Join, "tester2").to_string());

        assert_eq!(*game.game(), expected);
    }

    #[test]
    fn test_joins_too_many() {
        let mut game = ConnectFourClient::new(&"tester1".to_string());
        let expected = ConnectFour::new_with_players(&"tester1".to_string(), &"tester2".to_string());

        game.handle_message(&ConnectFourMessagePayload::new_from_str("SERVER".to_string(), Join, "tester1").to_string());
        game.handle_message(&ConnectFourMessagePayload::new_from_str("SERVER".to_string(), Join, "tester2").to_string());
        game.handle_message(&ConnectFourMessagePayload::new_from_str("SERVER".to_string(), Update, "tester3 failed to join").to_string());

        assert_eq!(*game.game(), expected);
    }


    #[test]
    fn test_update_game_once() {
        let mut game = ConnectFourClient::new(&"tester1".to_string());
        let mut expected = ConnectFour::new_with_players(&"tester1".to_string(), &"tester2".to_string());
        expected.make_move(0);

        game.handle_message(&ConnectFourMessagePayload::new_from_str("SERVER".to_string(), Join, "tester1").to_string());
        game.handle_message(&ConnectFourMessagePayload::new_from_str("SERVER".to_string(), Join, "tester2").to_string());

        game.handle_message(&ConnectFourMessagePayload::new_from_str("SERVER".to_string(), Update, "0").to_string());

        assert_eq!(*game.game(), expected);
    }

    #[test]
    fn test_update_game_twice() {
        let mut game = ConnectFourClient::new(&"tester1".to_string());
        let mut expected = ConnectFour::new_with_players(&"tester1".to_string(), &"tester2".to_string());
        expected.make_move(0);
        expected.make_move(3);

        game.handle_message(&ConnectFourMessagePayload::new_from_str("SERVER".to_string(), Join, "tester1").to_string());
        game.handle_message(&ConnectFourMessagePayload::new_from_str("SERVER".to_string(), Join, "tester2").to_string());

        game.handle_message(&ConnectFourMessagePayload::new_from_str("SERVER".to_string(), Update, "0").to_string());
        game.handle_message(&ConnectFourMessagePayload::new_from_str("SERVER".to_string(), Update, "3").to_string());

        assert_eq!(*game.game(), expected);
    }

    #[test]
    fn test_update_outofturn_game() {
        let mut game = ConnectFourClient::new(&"tester1".to_string());
        let mut expected = ConnectFour::new_with_players(&"tester1".to_string(), &"tester2".to_string());
        expected.make_move(0);

        game.handle_message(&ConnectFourMessagePayload::new_from_str("SERVER".to_string(), Join, "tester1").to_string());
        game.handle_message(&ConnectFourMessagePayload::new_from_str("SERVER".to_string(), Join, "tester2").to_string());

        game.handle_message(&ConnectFourMessagePayload::new_from_str("SERVER".to_string(), Update, "0").to_string());
        game.handle_message(&ConnectFourMessagePayload::new_from_str("tester1".to_string(), Update, "0").to_string());

        assert_eq!(*game.game(), expected);
    }


    #[test]
    fn test_exit() {
        let mut game = ConnectFourClient::new(&"tester1".to_string());
        let mut expected = ConnectFour::new_with_players(&"tester1".to_string(), &"tester2".to_string());
        expected.remove_player(&"tester1".to_string());

        game.handle_message(&ConnectFourMessagePayload::new_from_str("SERVER".to_string(), Join, "tester1").to_string());
        game.handle_message(&ConnectFourMessagePayload::new_from_str("SERVER".to_string(), Join, "tester2").to_string());

        game.handle_message(&ConnectFourMessagePayload::new_from_str("SERVER".to_string(), Exit, "tester1").to_string());

        assert_eq!(*game.game(), expected);
    }
}
