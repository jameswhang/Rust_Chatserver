use std::fmt;
use super::message::*;
use super::message::ConnectFourMType::*;
use super::super::{ConnectFour, Game, TurnBasedGame};


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

    pub fn handle_message(&mut self, message : &String) -> Result<Vec<String>, &str> {
        if let Some(payload) = ConnectFourMessagePayload::from_string(message) {
            match payload.m_type() {
                Join =>   Ok(self.handle_join(&payload).into_iter().map(|m| m.to_string()).collect::<Vec<String>>()),
                Update => Ok(self.handle_update(&payload).into_iter().map(|m| m.to_string()).collect::<Vec<String>>()),
                Exit =>   Ok(self.handle_exit(&payload).into_iter().map(|m| m.to_string()).collect::<Vec<String>>()),
                _ => unimplemented!(),
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
                ret.push(ConnectFourMessagePayload::new(&_SERVER_ID, Join, mcontent));
                ret.push(ConnectFourMessagePayload::new(&_SERVER_ID, Update, state.to_string()));
            },
            Err(s) => {
                ret.push(ConnectFourMessagePayload::new(&_SERVER_ID, Update, format!("{} failed to join", mcontent)))
            }
        }

        ret
    }

    fn handle_update(&mut self, message : &ConnectFourMessagePayload) -> Vec<ConnectFourMessagePayload> {
        let _SERVER_ID = SERVER_ID.to_string();
        let player_id = message.sender().clone();
        let mcontent = message.content().clone();
        let mut ret = vec![];

        //checks if the players in the game and if its their turn
        if !self.game.is_playing(&player_id) {
            ret.push(ConnectFourMessagePayload::new_from_str(player_id, Update, "You are not a player in this game"));
        } else if *self.game.whos_turn().id() != player_id {
            ret.push(ConnectFourMessagePayload::new_from_str(player_id, Update, "It is not your turn yet"));
        } else {
            if let Ok(col) = message.content().parse::<usize>() {
                match self.game.make_move(col) {
                    Ok(state) => { ret.push(ConnectFourMessagePayload::new(&_SERVER_ID, Update, mcontent)); },
                    Err(s) => { ret.push(ConnectFourMessagePayload::new_from_str(player_id, Update, "Invalid column attempted. Try again")); },
                }
            } else {
                ret.push(ConnectFourMessagePayload::new_from_str(player_id, Update, "Invalid column attempted. Try again"));
            }
        }

        ret
    }

    pub fn handle_exit(&mut self, message : &ConnectFourMessagePayload) -> Vec<ConnectFourMessagePayload> {
        let _SERVER_ID = SERVER_ID.to_string();
        let player_id = message.sender().clone();
        let mut ret = vec![];

        match self.game.remove_player(&player_id) {
            Ok(state) => {
                ret.push(ConnectFourMessagePayload::new(&_SERVER_ID, Exit, player_id));
            },

            Err(s) => {
                ret.push(ConnectFourMessagePayload::new_from_str(player_id, Update, "Invalid column attempted. Try again"));
            },
        }

        ret
    }

    pub fn game(&self) -> &ConnectFour {
        &self.game
    }
}



#[cfg(test)]
mod connectfourserver_test {
    use super::super::message::*;
    use super::super::super::*;
    use super::super::message::ConnectFourMType::*;

    #[test]
    fn test_joins_game() {
        let mut game = ConnectFourServer::new();
        let expected = ConnectFour::new_with_players(&"tester1".to_string(), &"tester2".to_string());

        game.handle_message(&ConnectFourMessagePayload::new_from_str("tester1".to_string(), Join, "tester1").to_string());
        game.handle_message(&ConnectFourMessagePayload::new_from_str("tester2".to_string(), Join, "tester2").to_string());

        assert_eq!(*game.game(), expected);
    }

    #[test]
    fn test_joins_too_many() {
        let mut game = ConnectFourServer::new();
        let expected = ConnectFour::new_with_players(&"tester1".to_string(), &"tester2".to_string());

        game.handle_message(&ConnectFourMessagePayload::new_from_str("tester1".to_string(), Join, "tester1").to_string());
        game.handle_message(&ConnectFourMessagePayload::new_from_str("tester2".to_string(), Join, "tester2").to_string());
        game.handle_message(&ConnectFourMessagePayload::new_from_str("tester3".to_string(), Join, "tester3").to_string());

        assert_eq!(*game.game(), expected);
    }

    #[test]
    fn test_join_string() {
        let mut game = ConnectFourServer::new();
        let expected = vec!["SERVER|JOIN|tester1".to_string(), "SERVER|UPDATE|FINISHED".to_string()];

        let results = game.handle_message(&ConnectFourMessagePayload::new_from_str("tester1".to_string(), Join, "tester1").to_string());

        assert_eq!(results.expect("Should convert"), expected);
    }

    #[test]
    fn test_update_game_once() {
        let mut game = ConnectFourServer::new();
        let mut expected = ConnectFour::new_with_players(&"tester1".to_string(), &"tester2".to_string());
        expected.make_move(0);

        game.handle_message(&ConnectFourMessagePayload::new_from_str("tester1".to_string(), Join, "tester1").to_string());
        game.handle_message(&ConnectFourMessagePayload::new_from_str("tester2".to_string(), Join, "tester2").to_string());

        game.handle_message(&ConnectFourMessagePayload::new_from_str("tester1".to_string(), Update, "0").to_string());

        assert_eq!(*game.game(), expected);
    }

    #[test]
    fn test_update_game_twice() {
        let mut game = ConnectFourServer::new();
        let mut expected = ConnectFour::new_with_players(&"tester1".to_string(), &"tester2".to_string());
        expected.make_move(0);
        expected.make_move(3);

        game.handle_message(&ConnectFourMessagePayload::new_from_str("tester1".to_string(), Join, "tester1").to_string());
        game.handle_message(&ConnectFourMessagePayload::new_from_str("tester2".to_string(), Join, "tester2").to_string());

        game.handle_message(&ConnectFourMessagePayload::new_from_str("tester1".to_string(), Update, "0").to_string());
        game.handle_message(&ConnectFourMessagePayload::new_from_str("tester2".to_string(), Update, "3").to_string());

        assert_eq!(*game.game(), expected);
    }

    #[test]
    fn test_update_outofturn_game() {
        let mut game = ConnectFourServer::new();
        let mut expected = ConnectFour::new_with_players(&"tester1".to_string(), &"tester2".to_string());
        expected.make_move(0);

        game.handle_message(&ConnectFourMessagePayload::new_from_str("tester1".to_string(), Join, "tester1").to_string());
        game.handle_message(&ConnectFourMessagePayload::new_from_str("tester2".to_string(), Join, "tester2").to_string());

        game.handle_message(&ConnectFourMessagePayload::new_from_str("tester1".to_string(), Update, "0").to_string());
        game.handle_message(&ConnectFourMessagePayload::new_from_str("tester1".to_string(), Update, "0").to_string());

        assert_eq!(*game.game(), expected);
    }

    #[test]
    fn test_exit() {
        let mut game = ConnectFourServer::new();
        let mut expected = ConnectFour::new_with_players(&"tester1".to_string(), &"tester2".to_string());
        expected.remove_player(&"tester1".to_string());

        game.handle_message(&ConnectFourMessagePayload::new_from_str("tester1".to_string(), Join, "tester1").to_string());
        game.handle_message(&ConnectFourMessagePayload::new_from_str("tester2".to_string(), Join, "tester2").to_string());

        game.handle_message(&ConnectFourMessagePayload::new_from_str("tester1".to_string(), Exit, "tester1").to_string());

        assert_eq!(*game.game(), expected);
    }

}
