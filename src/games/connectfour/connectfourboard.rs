use std::ops::Index;
use super::super::{TurnBasedGame, Player, GameState, MultiIndex, GResult};
use std::collections::HashMap;
use std::fmt;
use super::{NUM_ROWS, NUM_COLS};


/// Used to store the connect four board state
#[derive(Debug, PartialEq, Eq, Clone, )]
pub struct ConnectFourBoard {
    //a 6 by seven board
    //vector of vector
    spots : Vec<Vec<Option<Player>>>,
    //keeps track of what is free on the board
    free_spots : [usize ; NUM_COLS],
}

impl ConnectFourBoard {
    pub fn new() -> ConnectFourBoard {
        ConnectFourBoard {
            spots : vec![vec![ None ; NUM_COLS] ; NUM_ROWS],
            free_spots : [0; NUM_COLS]
        }
    }

    /// Adds a player's token to a column if possible
    /// @param col : usize - column to drop conenct four
    /// @param player : Player
    ///
    /// @return Result<(), &str>
    pub fn add_to_column(&mut self, col : usize, player : Player) -> GResult<&str> {
        if col >= 0 && col < NUM_COLS {
            let free_spot = self.free_spots[col];

            if free_spot < NUM_ROWS {
                self.set(MultiIndex(col, free_spot), Some(player.clone()));
                self.free_spots[col] = free_spot + 1;
                Ok(self.check_player(player))
            } else {
                Err("No space in column")
            }
        } else {
            Err("Invalid col index")
        }
    }

    pub fn inBounds(index : &MultiIndex) -> bool {
        !(index.0 < 0 || index.0 >= NUM_COLS || index.1 < 0 || index.1 >= NUM_ROWS)
    }

    pub fn set(&mut self, ind : MultiIndex, new_val : Option<Player>) -> bool {
        if ConnectFourBoard::inBounds(&ind) {
            self.spots[ind.1][ind.0] = new_val;
            true
        } else {
            false
        }
    }

    pub fn get_neighbors(&self, index: MultiIndex) -> Option<HashMap<MultiIndex, Option<Player>>> {
        if !ConnectFourBoard::inBounds(&index) {
            return None
        }

        let mut ret = HashMap::new();

        let xs : [usize ; 3] = [index.0 - 1, index.0, index.0 + 1];
        let ys : [usize ; 3] = [index.1 - 1, index.1, index.1 + 1];

        for x in xs.iter() {
            for y in ys.iter() {
                let check = MultiIndex(*x, *y);

                if ConnectFourBoard::inBounds(&check) {
                    ret.insert(check.clone(), self[check].clone());
                }
            }
        }

        Some(ret)
    }


    pub fn check_player(&self, player : Player) -> GameState {
        //checks for vertical wins
        for x in 0..NUM_COLS {
            // println!("Checking vertical line {}", x);
            if self.check_vertical(x, player.clone()) {
                return GameState::Finished;
            }
        }

        //checks for horizontal wins
        for y in 0..NUM_ROWS {
            // println!("Checking horizontal line {}", y);
            if self.check_horizontal(y, player.clone()) {
                return GameState::Finished;
            }
        }

        //checks for diagonal wins
        for x in 0..NUM_COLS {
            for y in 0..NUM_COLS {
                let ind = MultiIndex(x, y);
                // println!("Checking diagonal line {:?}", ind);

                if self.check_diagonal(ind, player.clone()) {
                    return GameState::Finished;
                }
            }
        }

        GameState::Ongoing
    }


    fn check_horizontal(&self, y : usize, player : Player) -> bool {
        let mut count = 0;

        for x in 0..NUM_COLS {
            let ind = MultiIndex(x, y);

            match self[ind] {
                Some(ref ptoken) if *ptoken == player => {
                    count = count + 1;
                },

                Some(_) => { count= 0; },

                _ => {},
            }
        }

        count >= 4
    }

    fn check_vertical(&self, x : usize, player : Player) -> bool {
        let mut count = 0;

        for y in 0..NUM_ROWS {
            let ind = MultiIndex(x, y);

            match self[ind] {
                Some(ref ptoken) if *ptoken == player => {
                    count = count + 1;
                },

                Some(_) => { count= 0; },

                _ => {},
            }
        }

        count >= 4
    }

    /// Checks only if the bottom-right/left has four in a row from the start
    fn check_diagonal(&self, start : MultiIndex, player : Player) -> bool {
        // Checks if start is too high to be the start of a diagonal for in the row
        if start.1 > NUM_ROWS - 4 {
            return false;
        }

        let mut count = 0;
        //check the right
        if start.0 < NUM_COLS - 4 {
            for offset in 0..4 {
                let ind = MultiIndex(start.0 + offset, start.1 + offset);

                match self[ind] {
                    Some(ref ptoken) if *ptoken == player => {
                        count = count + 1;
                    },

                    Some(_) => { count= 0; },

                    _ => {},
                }
            }

            if count >= 4 {
                return true;
            }
        }

        count = 0;

        //check the left
        if start.0 >=  3 {
            for offset in 0..4 {
                let ind = MultiIndex(start.0 - offset, start.1 + offset);

                match self[ind] {
                    Some(ref ptoken) if *ptoken == player => {
                        count = count + 1;
                    },

                    Some(_) => { count= 0; },

                    _ => {},
                }
            }

            if count >= 4 {
                return true;
            }
        }

        false
    }
}

/// Takes a tuple in as an index to index multi-dimensional ConnectFourBoard
impl Index<MultiIndex> for ConnectFourBoard {
    type Output = Option<Player>;

    fn index<'a>(&'a self, index: MultiIndex) -> &'a Option<Player> {
        &self.spots[index.1][index.0]
    }
}

/// Takes a tuple in as an index to index multi-dimensional ConnectFourBoard
impl Index<(usize, usize)> for ConnectFourBoard {
    type Output = Option<Player>;

    fn index<'a>(&'a self, index: (usize, usize)) -> &'a Option<Player> {
        &self.spots[index.1][index.0]
    }
}

impl fmt::Display for ConnectFourBoard {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut p = "".to_string();

        for row in &self.spots {
            let n  = row.iter().fold("".to_string(), |acc, ref player| format!("{}{:?}   ", acc, player).clone());
            p = n + "\n" + &p;
        }

        write!(f, "{}", p)
    }
}
