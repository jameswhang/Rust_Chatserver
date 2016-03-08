use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Hash, Clone, Debug, PartialEq)]
pub struct Dictionary {
    dict: Vec<String>,
}

impl Dictionary {
    pub fn initialize() -> Dictionary {
        // let mut dict = HashSet::<String>::new();
        let mut dict = vec![];

        let f = File::open("src/WORD.LST").expect("Error opening word lexicon!");
        let lines = BufReader::new(&f).lines();

        for line in lines {
            if let Ok(text) = line {
                // dict.insert(text.trim().to_owned());
                dict.push(text.trim().to_owned());
            }
        }
        Dictionary {
            dict: dict,
        }
    }

    pub fn check_word(&self, word: &str) -> bool {
        if self.dict.contains(&word.to_owned()) {
            return true;
        }
        false
    }
}
