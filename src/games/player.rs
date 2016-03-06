use std::fmt;
use super::Id;

// Implement Players
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum PlayerType {
    Computer,
    Human,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Player {
    p_type : PlayerType,
    id : Id,
}

impl Player {
    pub fn new(p_type : PlayerType, id : Id) -> Player {
        Player {
            p_type : p_type,
            id : id,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let p;

        match self.p_type {
            PlayerType::Computer => { p = "Comp: ".to_string() + &self.id },
            PlayerType::Human =>    { p = "Human: {}".to_string() + &self.id },
        }

        write!(f,"{}", p)
    }
}
