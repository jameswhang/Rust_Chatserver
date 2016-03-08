// use super::super::chat::message::Message;
use super::Player;
use std::fmt;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Ongoing,
    Finished,
}

impl fmt::Display for GameState {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameState::Ongoing => write!(f, "ONGOING"),
            GameState::Finished => write!(f, "FINISHED"),
        }
    }
}

pub trait Game {
    /// Returns whether the game has finished
    /// @return GameState
    fn is_done(&self) -> GameState;
    fn is_playing(&self, player_id : &Id) -> bool;
    fn is_full(&self) -> bool;

    fn get_players(&self) -> &[Player];
    /// returns the winner of the game if there is one
    fn get_winner(&self) -> Option<Player>;
    /// Get ranking of player in game
    fn get_position(&self, player: Player) -> Option<usize>;

    fn reset(&mut self);
}

// pub trait OnlineGame  : Game {
//     fn make_move_from(&mut self, player_move : Message) -> GResultChat;
// }

pub trait TurnBasedGame : Game {
    fn whos_turn(&self) -> Player;
}

pub type Id = String;
pub type GResult<T> = Result<GameState, T>;
pub type GResultChat = GResult<String>;
