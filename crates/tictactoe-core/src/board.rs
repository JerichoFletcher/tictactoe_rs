use super::mark::Mark;

pub struct Board {
    marks: [Mark; 9],
    turn: Mark,
    winner: Option<Mark>,
}

impl Board {
    const CHECK_ARRAY: &'static [usize; 3 * 4 * 9] = &[
        0, 1, 2, 0, 3, 6, 0, 4, 8, 9, 9, 9,
        0, 1, 2, 1, 4, 7, 9, 9, 9, 9, 9, 9,
        0, 1, 2, 2, 5, 8, 2, 4, 6, 9, 9, 9,
        3, 4, 5, 0, 3, 6, 9, 9, 9, 9, 9, 9,
        3, 4, 5, 1, 4, 7, 0, 4, 8, 2, 4, 6,
        3, 4, 5, 2, 5, 8, 9, 9, 9, 9, 9, 9,
        6, 7, 8, 0, 3, 6, 2, 4, 6, 9, 9, 9,
        6, 7, 8, 1, 4, 7, 9, 9, 9, 9, 9, 9,
        6, 7, 8, 2, 5, 8, 0, 4, 8, 9, 9, 9,
    ];

    pub fn new() -> Board {
        Self {
            marks: [Mark::Empty; 9],
            turn: Mark::X,
            winner: Option::None
        }
    }

    pub fn turn(&self) -> Mark {
        self.turn
    }

    pub fn winner(&self) -> Option<Mark> {
        self.winner
    }
    
    pub fn get(&self, row: usize, col: usize) -> Result<Mark, &str> {
        if row >= 3 || col >= 3 {
            return Err("Square is outside of the board");
        }

        let i = row * 3 + col;
        Ok(self.marks[i])
    }

    pub fn put(&mut self, row: usize, col: usize) -> Result<(), &str> {
        if row >= 3 || col >= 3 {
            return Err("Square is outside of the board");
        }
        if self.marks[row * 3 + col] != Mark::Empty {
            return Err("Square is not empty");
        }

        let i = row * 3 + col;
        self.marks[i] = self.turn;
        if !self.check_finish(row, col) {
            self.turn = self.turn.adversary();
        }

        Ok(())
    }

    fn check_finish(&mut self, row: usize, col: usize) -> bool {
        // Check for winner
        let i = row * 3 + col;
        let mut start = i * 3 * 4;
        while start < (i + 1) * 3 * 4 && Board::CHECK_ARRAY[start] < 9 {
            if
                self.turn == self.marks[Board::CHECK_ARRAY[start]] &&
                self.turn == self.marks[Board::CHECK_ARRAY[start + 1]] &&
                self.turn == self.marks[Board::CHECK_ARRAY[start + 2]]
            {
                self.winner = Option::Some(self.turn);
                return true;
            }
            start += 3;
        }

        // Check for tie
        let mut is_tie = true;
        for i in 0usize..9usize {
            if self.marks[i] == Mark::Empty { is_tie = false; break; }
        }
        if is_tie {
            self.winner = Option::Some(Mark::Empty);
            return true;
        }

        false
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}\n{}",
            format!("{}{}{}", self.marks[0], self.marks[1], self.marks[2]),
            format!("{}{}{}", self.marks[3], self.marks[4], self.marks[5]),
            format!("{}{}{}", self.marks[6], self.marks[7], self.marks[8]),
        )
    }
}
