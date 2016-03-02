use std::ops::Index;
use super::{Game, TurnBasedGame, Player, GameState};
use std::collections::HashMap;

#![allow(dead_code)]


const NUM_ROWS : usize = 4;
const NUM_COLS : usize = 4;

#[derive(Clone, Debug, PartialEq)]
struct BoggleBoard {
    //a 4x4 board
    spots : [[char; NUM_COLS] ; NUM_ROWS],
}

impl ConnectFourBoard {
    fn new() -> ConnectFourBoard {
        unimplemented!();

        ConnectFourBoard {
            spots : [['0' ; NUM_COLS] ; NUM_ROWS],
        }
    }
}

impl Index<(usize, usize)> for BoggleBoard {
    type Output = Option<Player>;

    fn index<'a>(&'a self, index: (usize, usize)) -> &'a Option<Player> {
        &self.sports[index.0][index.1]
    }
}



#[derive(PartialEq, Hash, Clone, Debug)]
pub struct Boggle {
    board: BoggleBoard,
    score : HashMap<Player, usize>, // two-player game
    valid_words : HashMap<String, bool>,
}

impl Boggle {
    pub fn new() -> Boggle {
        unimplemented!();
        //needs to generate words still
        Boggle {
            board: FourSquareBoard::new(),
            score : HashMap::new(),
        }
    }

    pub fn make_move(&mut self, word : String) GResult<&str> {
        match self.board.add_to_column(col, self.players[self.turn]) {
            Ok() => {
                if self.is_done() {
                    Ok(GameState::Finished)
                } else {
                    //switch to other player
                    self.turn = self.turn ^ 1;
                    Ok(GameState::Ongoing)
                }
            },

            Err(s) => Err(s),
        }
    }
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
    fn get_position(&self, player : Player) -> Option<usize> {
        match self.get_winner {
            Some(pl) if pl = player => Some(1),
            Some(pl) => {
                //calculate ranking
            },
            _ => None,
        }
    }

    fn reset(&mut self) {
        self.board = BoggleBoard::new(),

    }

    fn get_players(&self) -> &[Player] {
        &self.players
    }
}