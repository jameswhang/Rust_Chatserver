mod boggleboard;
use self::boggleboard::*

mod dictionary;
use self::dictionary::*

use std::ops::Index;
use std::io::{self, Read, stdin};
use super::{Game, TurnBasedGame, Player, GameState};
use std::collections::HashMap;

#[derive(PartialEq, Hash, Clone, Debug)]
pub struct Boggle {
    board: BoggleBoard,
    dict: Dictionary,
    // score : HashMap<Player, usize>, // two-player game
    // valid_words : HashMap<String, bool>,
}

impl Boggle {
    pub fn new() -> Boggle {
        Boggle {
            board: boggleboard::BoggleBoard::new(),
            dict : dictionary::Dictionary::initialize(),
        }
    }

    // pub fn make_move(&mut self, word : String) GResult<&str> {
    //     match self.board.add_to_column(col, self.players[self.turn]) {
    //         Ok() => {
    //             if self.is_done() {
    //                 Ok(GameState::Finished)
    //             } else {
    //                 //switch to other player
    //                 self.turn = self.turn ^ 1;
    //                 Ok(GameState::Ongoing)
    //             }
    //         },
    //
    //         Err(s) => Err(s),
    //     }
    // }
}


impl Game for Boggle{
    pub fn is_done(&self) -> bool {
        unimplemented!();
    }

    fn get_winner(&self) -> Option<Player> {
        if !self.is_done {
            return None;
        } else {
            //get the max of the scores
            unimplemented!();
        }
    }

    /// get ranking of player in game
    // fn get_position(&self, player : Player) -> Option<usize> {
    //     match self.get_winner {
    //         Some(pl) if pl = player => Some(1),
    //         Some(pl) => {
    //             //calculate ranking
    //         },
    //         _ => None,
    //     }
    // }

    fn reset(&mut self) {
        self.board = BoggleBoard::new(),
    }

    fn get_players(&self) -> &[Player] {
        &self.players
    }
}
