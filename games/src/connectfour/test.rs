#[cfg(test)]
mod connectfour_tests {
    use super::*;
    use super::super::*;

    #[test]
    fn win_vertical() {
        let mut cg = get_test_board();

        assert_eq!(cg.make_move(0).expect("impossible"), GameState::Finished);
    }

    #[test]
    fn win_vertical1() {
        let mut cg = get_test_board();

        assert_eq!(cg.make_move(0).expect("impossible"), GameState::Finished);
    }

    #[test]
    fn win_vertical2() {
        let mut cg = get_test_board();
        cg.make_move(2);
        assert_eq!(cg.make_move(1).expect("impossible"), GameState::Finished);
    }

    #[test]
    fn win_horizontal() {
        let mut cg = ConnectFour::new("player1".to_string(), "player2".to_string());
        assert_eq!(cg.make_move(0).expect("impossible"), GameState::Ongoing);
        assert_eq!(cg.make_move(0).expect("impossible"), GameState::Ongoing);
        assert_eq!(cg.make_move(1).expect("impossible"), GameState::Ongoing);
        assert_eq!(cg.make_move(1).expect("impossible"), GameState::Ongoing);
        assert_eq!(cg.make_move(2).expect("impossible"), GameState::Ongoing);
        assert_eq!(cg.make_move(2).expect("impossible"), GameState::Ongoing);

        assert_eq!(cg.make_move(3).expect("impossible"), GameState::Finished);
    }

    fn get_test_board() -> ConnectFour {
        let mut cg = ConnectFour::new("player1".to_string(), "player2".to_string());

        cg.make_move(0);
        cg.make_move(1);
        cg.make_move(0);
        cg.make_move(1);
        cg.make_move(0);
        cg.make_move(1);
        cg
    }
}
