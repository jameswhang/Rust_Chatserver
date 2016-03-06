use std::fmt;
use super::super::*;


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ConnectFourClient<T> {
    game : ConnectFour,
}

impl ConnectFourClient<T> where T : Read + Write + Clone {
    pub fn new(connection : T) -> ConnectFourClient<T> {
        ConnectFourClient {
            game : ConnectFour::new(),
            connection : connection.clone()
        }
    }

    pub fn make_move
}
