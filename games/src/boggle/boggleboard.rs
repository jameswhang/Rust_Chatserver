use std::fmt;

extern crate rand;
use self::rand::distributions::{IndependentSample, Range};

const NUM_ROWS: usize = 4;
const NUM_COLS: usize = 4;
const DIES: [[&'static str; 6]; 16] = [["R", "I", "F", "O", "B", "X"],
                                      ["I", "F", "E", "H", "E", "Y"],
                                      ["D", "E", "N", "O", "W", "S"],
                                      ["U", "T", "O", "K", "N", "D"],
                                      ["H", "M", "S", "R", "A", "O"],
                                      ["L", "U", "P", "E", "T", "S"],
                                      ["A", "C", "I", "T", "O", "A"],
                                      ["Y", "L", "G", "K", "U", "E"],
                                      ["QU", "B", "M", "J", "O", "A"],
                                      ["E", "H", "I", "S", "P", "N"],
                                      ["V", "E", "T", "I", "G", "N"],
                                      ["B", "A", "L", "I", "Y", "T"],
                                      ["E", "Z", "A", "V", "N", "D"],
                                      ["R", "A", "L", "E", "S", "C"],
                                      ["U", "W", "I", "L", "R", "G"],
                                      ["P", "A", "C", "E", "M", "D"]];


#[derive(Hash, Copy, Clone, Debug, PartialEq)]
pub struct BoggleBoard {
    //a 4x4 board
    spots : [[&'static str; NUM_COLS] ; NUM_ROWS],
    visited : [[bool; NUM_COLS]; NUM_ROWS],
}

impl BoggleBoard {
    pub fn new() -> BoggleBoard {
        // randomize order of dies
        let mut used_dies = vec![];
        let possible_dies = Range::new(0, 16);
        let mut rng = rand::thread_rng();
        while used_dies.len() < 16 {
            let random_die = possible_dies.ind_sample(&mut rng);
            if !used_dies.contains(&random_die) {
                used_dies.push(random_die);
            }
        }

        // for each die, pick random face and insert into new board
        let mut insert_letters = vec![];
        for die in used_dies {
            // get random die face
            let possible_die_faces = Range::new(0, 6);
            let mut rng = rand::thread_rng();
            let die_face = possible_die_faces.ind_sample(&mut rng);

            // get letter
            let insert_letter = DIES[die][die_face];
            &insert_letters.push(insert_letter);
        }

        // make boggle board
        BoggleBoard {
            spots : [[insert_letters[0], insert_letters[1], insert_letters[2], insert_letters[3]],
                     [insert_letters[4], insert_letters[5], insert_letters[6], insert_letters[7]],
                     [insert_letters[8], insert_letters[9], insert_letters[10], insert_letters[11]],
                     [insert_letters[12], insert_letters[13], insert_letters[14], insert_letters[15]]],
            visited: [[false; NUM_COLS]; NUM_ROWS],
        }
    }

    fn reset_visits(&mut self) {
        self.visited = [[false; NUM_COLS]; NUM_ROWS];
    }

    pub fn check_word(&mut self, word: &str) -> bool {
        // check to see if word valid and transform into vector of strs
        let mut word_vec = vec![];
        let word_len = word.len();
        let mut word_iter = word.chars();
        if word_len > 0 {
            for _ in 0..word.len() {
                &word_vec.push(word_iter.next().unwrap().to_string().to_uppercase());
            }
        } else {
            println!("Word is invalid.");
            return false;
        }

        // see if first letter exists in grid
        let first_letter = word_vec[0].clone();
        let mut first_letter_matches = vec![];
        for row in 0..4 {
            for col in 0..4 {
                if &first_letter == self.spots[row][col] {
                    first_letter_matches.push((row, col));
                }
            }
        }

        // if yes, BFS to find matching word
        if first_letter_matches.len() > 0 {

            // check every possible first letter
            for first_letter_idx in first_letter_matches {

                // reset visited spots, visit myself
                BoggleBoard::reset_visits(self);
                self.visited[first_letter_idx.0][first_letter_idx.1] = true;

                let mut queue = vec![];
                queue.push((first_letter_idx, 1));

                // BFS
                while queue.len() > 0 {
                    let idx = queue.pop().unwrap();

                    if idx.1 == word_len {
                        return true;
                    }
                    let neighbors = BoggleBoard::find_unvisited_neighbors(self, idx.0);
                    for item in neighbors {
                        if self.spots[item.0][item.1] == word_vec[idx.1] {
                            self.visited[item.0][item.1] = true;
                            queue.push((item, idx.1 + 1));
                        }
                    }
                }
            }
        }
        false
    }

    fn find_unvisited_neighbors(&self, idx: (usize, usize)) -> Vec<(usize, usize)> {
        let mut unvisited = vec![];

        for i in 0..3 {
            for j in 0..3 {
                let temp_idx: (isize, isize) = (idx.0 as isize + i - 1, idx.1 as isize + j - 1);
                if BoggleBoard::check_valid_index(temp_idx){
                    //recast to usize
                    let temp_idx: (usize, usize) = (temp_idx.0 as usize, temp_idx.1 as usize);
                    if self.visited[temp_idx.0][temp_idx.1] != true {
                        unvisited.push(temp_idx);
                    }
                }
            }
        }
        unvisited
    }

    fn check_valid_index(idx: (isize, isize)) -> bool {
        if idx.0 >= 0 && idx.0 < 4 && idx.1 >= 0 && idx.1 < 4 {
            return true;
        }
        false
    }
}

impl fmt::Display for BoggleBoard {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut p = "".to_string();

        for row in 0..4 {
            for col in 0..4 {
                p = p + &self.spots[row][col] + " ";
            }
            p = p + "\n"
        }

        write!(f, "{}", p)
    }
}
