pub mod board;
pub mod mark;

#[cfg(test)]
mod tests {
    use crate::{board::Board, mark::Mark};

    #[test]
    fn when_new_board_then_empty() {
        let board = Board::new();
        for i in 0..9 {
            match board.get(i / 3, i % 3) {
                Ok(mark) => assert_eq!(Mark::Empty, mark),
                Err(e) => panic!("{e}")
            }
        }
    }

    #[test]
    fn when_put_on_board_then_pass_turn() {
        let mut board = Board::new();
        let mut i = 0;
        while i < 9 && board.winner().is_none() {
            let curr = board.turn();
            match board.put(i / 3, i % 3) {
                Ok(_) => if board.winner().is_none() {
                    assert_eq!(curr.adversary(), board.turn())
                },
                Err(e) => panic!("{e}")
            }
            i += 1;
        }
    }

    #[test]
    fn when_put_on_board_then_can_get() {
        let mut board = Board::new();
        let mut i = 0;
        while i < 9 && board.winner().is_none() {
            let curr = board.turn();
            match board.put(i / 3, i % 3) {
                Ok(_) => match board.get(i / 3, i % 3) {
                    Ok(mark) => assert_eq!(curr, mark),
                    Err(e) => panic!("{e}")
                },
                Err(e) => panic!("{e}")
            }
            i += 1;
        }
    }
}
