use super::types::{Id};
pub mod connecfour;
//pub mod boggle;

#[derive(Clone, PartialEq)]
pub enum Player {
    Nil,
    Comp(String),
    Human(Id),
}

#[derive(Clone, PartialEq)]
pub enum GameState {
    Ongoing,
    Finished,
}

pub trait Game {
    fn is_done(&self) -> bool;
    fn get_winner(&self) -> Option<Player>;
    fn get_position(&self, player: Player) -> Option<usize>;
    fn reset(&mut self);
    fn get_players(&self) -> &[Player];
}

pub trait TurnBasedGame : Game {
    fn whos_turn(&self) -> Player;
}


pub type GResult<T> = Result<GameState, T>;
