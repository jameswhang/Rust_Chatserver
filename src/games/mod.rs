use super::chat::message::Message;
use std::fmt;

pub mod connectfour;
//pub mod boggle;

pub type Id = String;


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Player {
    Nil,
    Comp(Id),
    Human(Id),
}

impl fmt::Display for Player {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let p;

        match self {
            &Player::Nil => { p = "Nil".to_string() },

            &Player::Comp(ref id) => {p = "Comp: ".to_string() + &id },

            &Player::Human(ref id) => {p = "Human: {}".to_string() + &id },
        }

        write!(f,"{}", p)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Ongoing,
    Finished,
}

pub trait Game {
    /// Returns whether the game has finished
    fn is_done(&self) -> GameState;
    /// returns the winner of the game if there is one
    fn get_winner(&self) -> Option<Player>;
    /// Get ranking of player in game
    fn get_position(&self, player: Player) -> Option<usize>;
    fn reset(&mut self);
    fn get_players(&self) -> &[Player];
}

pub trait OnlineGame  : Game {
    fn make_move_from(&mut self, player_move : Message) -> GResultChat;
}

pub trait TurnBasedGame : Game {
    fn whos_turn(&self) -> Player;
}


pub type GResult<T> = Result<GameState, T>;
pub type GResultChat = GResult<String>;


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct MultiIndex (usize, usize);

impl MultiIndex {
    fn is_vertical(i1 : MultiIndex, i2 : MultiIndex) -> bool {
        let beside = i1.0 == i2.0;
        let stacked = i1.1 + 1 == i2.1 || i1.1 - 1 == i2.1;
        beside && stacked
    }

    fn is_horizontal(i1 : MultiIndex, i2 : MultiIndex) -> bool {
        let beside = i1.0 + 1 == i2.0 || i1.0 - 1 == i2.0;
        let stacked = i1.1  == i2.1 ;
        beside && stacked
    }

    fn is_diagonal(i1 : MultiIndex, i2 : MultiIndex) -> bool {
        let beside = i1.0 + 1 == i2.0 || i1.0 - 1 == i2.0;
        let stacked = i1.1 + 1 == i2.1 || i1.1 - 1 == i2.1;
        beside && stacked
    }
}

impl Into<(usize, usize)> for MultiIndex {
    fn into(self) -> (usize, usize) {
        (self.0, self.1)
    }
}

impl Into<MultiIndex> for (usize, usize) {
    fn into(self) -> MultiIndex {
        MultiIndex(self.0, self.1)
    }
}
