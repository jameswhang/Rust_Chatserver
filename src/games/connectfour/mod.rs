pub mod connectfourboard;
// pub mod onlineconnectfour;

use std::ops::Index;
use std::collections::HashMap;
use std::fmt;

use super::{Game, TurnBasedGame, Player, GameState, GResult, GResultChat, MultiIndex, Id};
use super::PlayerType::*;
use self::connectfourboard::*;

pub const NUM_ROWS : usize = 6;
pub const NUM_COLS : usize = 7;

/// Struct to play the game - ConnectFour
#[derive(PartialEq, Clone, Debug)]
pub struct ConnectFour {
    board: ConnectFourBoard,
    players : Vec<Player>, // two-player game
    turn : usize,
}

impl ConnectFour {
    pub fn new(id1 : String, id2 : String) -> ConnectFour {
        ConnectFour{
            board: ConnectFourBoard::new(),
            players: vec![Player::new(Human, id1), Player::new(Human, id2)],
            turn : 0,
        }
    }

    pub fn make_move(&mut self, col : usize) -> GResult<&str> {
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


#[cfg(test)]
mod connectfour_tests {
    use super::*;
    use super::super::*;

    #[test]
    fn win_vertical() {
        let mut cg = get_test_board();

        assert_eq!(cg.make_move(0).expect("impossible"), GameState::Finished);
    }

    #[test]
    fn win_vertical1() {
        let mut cg = get_test_board();

        assert_eq!(cg.make_move(0).expect("impossible"), GameState::Finished);
    }

    #[test]
    fn win_vertical2() {
        let mut cg = get_test_board();
        cg.make_move(2);
        assert_eq!(cg.make_move(1).expect("impossible"), GameState::Finished);
    }

    #[test]
    fn win_horizontal() {
        let mut cg = ConnectFour::new("player1".to_string(), "player2".to_string());
        assert_eq!(cg.make_move(0).expect("impossible"), GameState::Ongoing);
        assert_eq!(cg.make_move(0).expect("impossible"), GameState::Ongoing);
        assert_eq!(cg.make_move(1).expect("impossible"), GameState::Ongoing);
        assert_eq!(cg.make_move(1).expect("impossible"), GameState::Ongoing);
        assert_eq!(cg.make_move(2).expect("impossible"), GameState::Ongoing);
        assert_eq!(cg.make_move(2).expect("impossible"), GameState::Ongoing);

        assert_eq!(cg.make_move(3).expect("impossible"), GameState::Finished);
    }

    fn get_test_board() -> ConnectFour {
        let mut cg = ConnectFour::new("player1".to_string(), "player2".to_string());

        cg.make_move(0);
        cg.make_move(1);
        cg.make_move(0);
        cg.make_move(1);
        cg.make_move(0);
        cg.make_move(1);
        cg
    }
}
