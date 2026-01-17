#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn other(self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    pub fn mark(self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
        }
    }
}