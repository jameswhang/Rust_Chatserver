extern crate rand;
use rand::distributions::{IndependentSample, Range};

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


#[derive(Clone, Debug, PartialEq)]
pub struct BoggleBoard {
    //a 4x4 board
    board : [[&'static str; NUM_COLS] ; NUM_ROWS],
}

impl BoggleBoard {
    fn new() -> BoggleBoard {
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
            board : [[insert_letters[0], insert_letters[1], insert_letters[2], insert_letters[3]],
                     [insert_letters[4], insert_letters[5], insert_letters[6], insert_letters[7]],
                     [insert_letters[8], insert_letters[9], insert_letters[10], insert_letters[11]],
                     [insert_letters[12], insert_letters[13], insert_letters[14], insert_letters[15]]],
        }
    }

    fn display(self) {
        for row in 0..4 {
            let mut line = "".to_string();
            for col in 0..4 {
                line = line + self.board[row][col] + " ";
            }
            println!("{}", line);
        }
    }
}


fn main() {
    println!("Hello, world!");
    let boggle_board = BoggleBoard::new();
    boggle_board.display();
}
