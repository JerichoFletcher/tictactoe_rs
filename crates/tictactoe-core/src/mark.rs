#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Mark {
    Empty,
    X,
    O,
}

impl Mark {
    pub fn adversary(&self) -> Mark {
        match self {
            Mark::X => Mark::O,
            Mark::O => Mark::X,
            Mark::Empty => Mark::Empty,
        }
    }
}

impl std::fmt::Display for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Mark::Empty => "_",
            Mark::X => "X",
            Mark::O => "O"
        })
    }
}
