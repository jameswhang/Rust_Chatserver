use super::super::chat::message::Message;
use super::Player;

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

pub type Id = String;
pub type GResult<T> = Result<GameState, T>;
pub type GResultChat = GResult<String>;
