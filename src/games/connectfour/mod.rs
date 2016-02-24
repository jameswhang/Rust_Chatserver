use std::ops::Index;
use super::{Game, TurnBasedGame, Player, GameState, GResult};

#![allow(dead_code)]

const NUM_ROWS : usize = 6;
const NUM_COLS : usize = 7;

#[derive(Clone, Debug, PartialEq)]
struct ConnectFourBoard {
    //a 6 by seven board
    spots : [[Option<Player>; NUM_COLS] ; NUM_ROWS],
    //keeps track of what is free on the board
    free_spots : [usize ; NUM_COLS],
}

impl ConnectFourBoard {
    fn new() -> ConnectFourBoard {
        ConnectFourBoard {
            spots : [[None ; NUM_COLS] ; NUM_ROWS],
            free_spots : [0; NUM_COLS]
        }
    }

    fn add_to_column(&mut self, col : usize, player : Player) -> Result<(), &str> {
        if col < NUM_COLS {
            let free_spot = self.free_spots[col];

            if free_spot < NUM_ROWS {
                self.board[(free_spot, col)] = Some(player);
                Ok()
            } else {
                Err("No space in column")
            }
        }
    }
}

impl Index<(usize, usize)> for ConnectFourBoard {
    type Output = Option<Player>;

    fn index<'a>(&'a self, index: (usize, usize)) -> &'a Option<Player> {
        &self.sports[index.0][index.1]
    }
}



#[derive(PartialEq, Hash, Clone, Debug)]
pub struct ConnectFour {
    board: FourSquareBoard,
    players : [Player ; 2], // two-player game
    turn : usize,
}

impl ConnectFour {
    pub fn new() -> FourSquareBoard {
        FourSquare{
            board: FourSquareBoard::new(),
            players: [Player::Human(0), Player::Human(1)],
            turn : 0,
        }
    }

    pub fn make_move(&mut self, col : usize) GResult<&str> {
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


impl Game for ConnectFour{
    pub fn is_done(&self) -> bool {
        unimplemented!();
    }

    fn get_winner(&self) -> Option<Player> {
        if !self.is_done {
            return None;
        } else {
            //last person to make a move must have won
            Some(self.players[self.turn])
        }
    }

    /// get ranking of player in game
    fn get_position(&self, player : Player) -> Option<usize> {
        match self.get_winner {
            Some(pl) if pl = player => Some(1),
            Some(pl) => Some(2),
            _ => None,
        }
    }

    fn reset(&mut self) {
        self.board = FourSquareBoard::new(),
        self.turn = 0;
    }

    fn get_players(&self) -> &[Player] {
        &self.players
    }
}

impl TurnBasedGame for ConnectFour {
    fn whos_turn(&self) -> Player {
        self.players[self.turn]
    }
}
