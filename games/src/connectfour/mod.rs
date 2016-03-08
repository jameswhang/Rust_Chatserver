pub mod connectfourboard;
pub use self::connectfourboard::*;

pub mod onlineconnectfour;
pub use self::onlineconnectfour::{ConnectFourServer, ConnectFourClient};
// mod test;

use std::ops::Index;
use std::collections::HashMap;
use std::fmt;

pub use super::{Game, TurnBasedGame, Player, GameState, GResult, GResultChat, MultiIndex, Id};
use super::PlayerType::*;

pub const NUM_ROWS : usize = 6;
pub const NUM_COLS : usize = 7;

/// Struct to play the game - ConnectFour
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ConnectFour {
    board: ConnectFourBoard,
    players : Vec<Player>, // two-player game
    turn : usize,
}

impl ConnectFour {
    pub fn new() -> ConnectFour {
        ConnectFour{
            board: ConnectFourBoard::new(),
            players: vec![],
            turn : 0,
        }
    }

    pub fn new_with_players(id1 : &String, id2 : &String) -> ConnectFour {
        ConnectFour{
            board: ConnectFourBoard::new(),
            players: vec![Player::new(Human, id1.clone()), Player::new(Human, id2.clone())],
            turn : 0,
        }
    }

    pub fn make_move(&mut self, col : usize) -> GResult<&str> {
        if !self.is_full() {
            return Err("Not enough players");
        }

        let result = self.board.add_to_column(col, self.players[self.turn].clone());

        match result {
            Ok(state) => {
                if state == GameState::Finished {
                    // don't switch so we know who won
                    Ok(GameState::Finished)
                } else {
                    // switch to other player by XORing
                    self.turn = self.turn ^ 1;
                    Ok(GameState::Ongoing)
                }
            },

            Err(s) => Err(s),
        }
    }

    pub fn add_player(&mut self, id : &String) -> GResult<&str> {
        if !self.is_full() {
            self.players.push(Player::new(Human, id.clone()));
            if self.is_full() {
                Ok(GameState::Ongoing)
            } else {
                Ok(GameState::Finished)
            }
        } else {
            Err("Game is full")
        }
    }

    pub fn remove_player(&mut self, id : &String) -> GResult<&str> {
        let plen = self.players.len();
        let mut remove_ind = -1;

        for ind in 0..plen {
            if *self.players[ind].id() == *id {
                remove_ind = ind as i32;
                break;
            }
        }

        if remove_ind >= 0 {
            self.players.remove(remove_ind as usize);
            Ok(GameState::Finished)
        } else {
            Err("Player not found in game")
        }
    }
}


impl Game for ConnectFour{
    fn is_done(&self) -> GameState {
        for player in &self.players {
            if self.board.check_player(player.clone()) == GameState::Finished {
                return GameState::Finished;
            }
        }

        GameState::Ongoing
    }


    fn is_playing(&self, player_id : &Id) -> bool {
        self.players.iter().filter(|x| *x.id() == *player_id).size_hint().1.unwrap() > 0
    }


    fn is_full(&self) -> bool {
        self.players.len() == 2
    }


    fn get_winner(&self) -> Option<Player> {
        if self.is_done() == GameState::Finished {
            //last person to make a move must have won
            Some(self.players[self.turn].clone())
        }
        else {
            return None;
        }
    }

    fn get_position(&self, player : Player) -> Option<usize> {
        match self.get_winner() {
            Some(pl) => {
                if pl == player {
                    Some(1)
                } else {
                    Some(2)
                }
            },

            _ => None,
        }
    }

    fn reset(&mut self) {
        self.board = ConnectFourBoard::new();
        self.turn = 0;
    }

    /// @return &[Player]
    fn get_players(&self) -> &[Player] {
        &self.players
    }

}

impl TurnBasedGame for ConnectFour {
    fn whos_turn(&self) -> Player {
        self.players[self.turn].clone()
    }
}

impl fmt::Display for ConnectFour {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.board)
    }
}
