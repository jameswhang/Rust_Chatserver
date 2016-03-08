#[cfg(test)]
mod boggle_tests {
    use super::*;
    use super::super::*;
    use dictionary::*;
    use boggleboard::*;

    #[test]
    fn check_word_horizontal_vertical() {
        let mut board = get_test_board();
        assert_eq!(board.check_word("ABCD"), true);
        assert_eq!(board.check_word("AEFG"), true);
    }

    #[test]
    fn check_word_case_insensitive() {
        let mut board = get_test_board();
        assert_eq!(board.check_word("abCd"), true);
        assert_eq!(board.check_word("AeFg"), true);
    }

    #[test]
    fn check_word_circular() {
        let mut board = get_test_board();
        assert_eq!(board.check_word("abhe"), true);
    }

    fn check_word_no_repeat() {
        let mut board = get_test_board();
        assert_eq!(board.check_word("Abcdc"), false);
    }

    #[test]
    fn check_dict_word() {
        let dict = get_dict();
        assert_eq!(dict.check_word("hello"), true);
        assert_eq!(dict.check_word("world"), true);
        assert_eq!(dict.check_word("hasdfdafssafd"), false);
        assert_eq!(dict.check_word("gregaerwe"), false);
    }

    fn get_test_board() -> BoggleBoard {
        let mut boggle_board = BoggleBoard::new();
        boggle_board.spots[0][0] = "A";
        boggle_board.spots[0][1] = "B";
        boggle_board.spots[0][2] = "C";
        boggle_board.spots[0][3] = "D";
        boggle_board.spots[1][0] = "E";
        boggle_board.spots[2][0] = "F";
        boggle_board.spots[3][0] = "G";
        boggle_board.spots[1][1] = "H";
        boggle_board.spots[1][2] = "I";
        boggle_board.spots[1][3] = "J";
        boggle_board.spots[2][1] = "K";
        boggle_board.spots[2][2] = "L";
        boggle_board.spots[2][3] = "M";
        boggle_board.spots[3][1] = "N";
        boggle_board.spots[3][2] = "O";
        boggle_board.spots[3][3] = "P";
        boggle_board
    }

    fn get_dict() -> Dictionary {
        let dict = Dictionary::initialize();
        dict
    }
}
