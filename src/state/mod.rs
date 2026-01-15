
use std::io::{self, Write};
use crate::ui;

#[derive(Debug, PartialEq, Eq)]
pub struct GameState {
    board: [Option<char>; 9],
    current_player: Player,
    screen_state: ScreenState,
    game_over: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Player { X, O }

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ScreenState {
    InGame,
    GameOver,
    TieGame,
}

impl ScreenState {
    pub fn render(
        &self,
        state: &GameState,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        match self {
            ScreenState::InGame => {
                ui::display_board(state.board(), &mut *out)?;
            }
            ScreenState::GameOver => {
                ui::announce_winner(state.current_player.other().mark(), &mut *out)?;
            }
            ScreenState::TieGame => {
                ui::announce_tie(&mut *out)?;
            }
        }
        out.flush()
    }
}

impl GameState {
    pub const BOARD_SIZE: usize = 9;

    pub fn new() -> Self {
        Self {
            board: [None; Self::BOARD_SIZE],
            current_player: Player::X,
            screen_state: ScreenState::InGame,
            game_over: false,
        }
    }

    pub fn board(&self) -> &[Option<char>; Self::BOARD_SIZE] {
        &self.board
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn render_screen(
        &self,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        self.screen_state.render(self, out)
    }

    pub fn is_game_over(&self) -> bool {
        matches!(self.screen_state, ScreenState::GameOver)
    }

    pub fn is_tie(&self) -> bool {
        matches!(self.screen_state, ScreenState::TieGame)
    }

    pub fn game_over(&mut self) {
        self.screen_state = ScreenState::GameOver;
    }

    pub fn tie_game(&mut self) {
        self.screen_state = ScreenState::TieGame;
    }

    pub fn next_state(&mut self, idx: usize) {
        self.board[idx] = Some(self.current_player.mark());
        self.switch_player();
    }

    pub fn switch_player(&mut self) {
        self.current_player = self.current_player.other();
    }
}

#[cfg(test)]
mod spec;
