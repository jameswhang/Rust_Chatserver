pub mod boggleboard;
use self::boggleboard::*;

pub mod dictionary;
use self::dictionary::*;

use std::fmt;
use std::io::{Read};
use super::{Player};
use super::PlayerType::*;

#[derive(PartialEq, Hash, Clone, Debug)]
pub struct Boggle {
    board: BoggleBoard,
    dict: Dictionary,
    players : Vec<Player>,
}

impl Boggle {
    pub fn new() -> Boggle {
        Boggle {
            board: boggleboard::BoggleBoard::new(),
            dict : dictionary::Dictionary::initialize(),
            players : vec![],
        }
    }

    pub fn new_with_players(id1: &String, id2: &String) -> Boggle {
        Boggle {
            board: boggleboard::BoggleBoard::new(),
            dict : dictionary::Dictionary::initialize(),
            players : vec![Player::new(Human, id1.clone()), Player::new(Human, id2.clone())],
        }
    }
}

impl fmt::Display for Boggle {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.board)
    }
}
