#[derive(Debug, Clone)]

pub struct GameState {
    pub cells: [Option<char>; 9],
    pub current_player: char,
    pub game_over: bool,
}

impl GameState {
    pub const P1: char = 'X';
    pub const P2: char = 'O';

    pub fn new() -> Self {
        Self {
            cells: [None; 9],
            current_player: Self::P1,
            game_over: false,
        }
    }

    pub fn maybe_apply_move(&mut self, idx: usize, on_error: impl FnOnce()) {
        if self.cells[idx].is_some() {
            on_error();
            return;
        }
        self.cells[idx] = Some(self.current_player);
    }

    fn switch_player(&mut self) {
        self.current_player = if self.current_player == Self::P1 {
            Self::P2
        } else {
            Self::P1
        };
    }
}

#[cfg(test)]
mod game_spec;
