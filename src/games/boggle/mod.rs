mod boggleboard;
use self::boggleboard::*;

mod dictionary;
use self::dictionary::*;

use std::ops::Index;
use std::io::{self, Read, stdin};
use super::{Game, TurnBasedGame, Player, GameState, GResult, Id};
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

    pub fn add_player(&mut self, id: &String) -> GResult<&str> {
        if !self.is_full() {
            self.players.push(Player::new(Human, id.clone()));
            if self.is_full() {
                Ok(GameState::Ongoing)
            } else {
                Ok(GameState::Finished)
            }
        } else {
            Err("Game is full")
        }
    }

    pub fn remove_player(&mut self, id : &String) -> GResult<&str> {
        let plen = self.players.len();
        let mut remove_ind = -1;

        for ind in 0..plen {
            if *self.players[ind].id() == *id {
                remove_ind = ind as i32;
                break;
            }
        }

        if remove_ind >= 0 {
            self.players.remove(remove_ind as usize);
            Ok(GameState::Finished)
        } else {
            Err("Player not found in game")
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
    pub fn is_done(&self) -> GameState {
        for player in &self.players {
            if self.board.check_player(player.clone()) == GameState::Finished {
                return GameState::Finished;
            }
        }
        GameState::Ongoing
    }

    fn is_playing(&self, player_id: &Id) -> bool {
        self.players.iter().filter(|x| *x.id() == *player_id).size_hint().1.unwrap() > 0
    }

    fn is_full(&self) -> bool {
        self.players.len() == 2
    }

    fn get_winner(&self) -> Option<Player> {
        if !self.is_done {
            return None;
        } else {
            //get the max of the scores
            unimplemented!();
        }
    }

    // get ranking of player in game
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
        self.board = BoggleBoard::new();
    }

    fn get_players(&self) -> &[Player] {
        &self.players
    }
}
