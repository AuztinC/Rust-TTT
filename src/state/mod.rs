
use std::io::{self, Write};
use crate::ui;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameState {
    board: [Option<char>; 9],
    current_player: Player,
    screen_state: ScreenState,
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

// pub trait Agent {
//     fn select_move(&mut self, state: &GameState) -> usize;
// }

// pub struct AiAgent;

// impl Agent for AiAgent {
//     fn select_move(&mut self, state: &GameState) -> usize {
//         crate::ai::ai_move(state)
//     }
// }

// pub struct Match<X: Agent, O: Agent> {
//     pub x: X,
//     pub o: O,
// }

// impl<X: Agent, O: Agent> Match<X, O> {
//     pub fn select_move(&mut self, state: &GameState) -> usize {
//         match state.current_player {
//             Player::X => self.x.select_move(state),
//             Player::O => self.o.select_move(state),
//         }
//     }
// }



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
                ui::display_board(state.board(), &mut *out)?;
                ui::announce_winner(state.current_player.other().mark(), &mut *out)?;
            }
            ScreenState::TieGame => {
                ui::display_board(state.board(), &mut *out)?;
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
        }
    }

    pub fn board(&self) -> &[Option<char>; Self::BOARD_SIZE] {
        &self.board
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn screen_state(&self) -> ScreenState {
        self.screen_state
    }

    pub fn render_screen(
        &self,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        self.screen_state.render(self, out)
    }

    pub fn _is_game_over(&self) -> bool {
        matches!(self.screen_state, ScreenState::GameOver)
    }

    pub fn _is_tie(&self) -> bool {
        matches!(self.screen_state, ScreenState::TieGame)
    }

    pub fn game_over(&mut self) -> Self {
        self.screen_state = ScreenState::GameOver;
        self.clone()
    }

    pub fn tie(&mut self) -> Self {
        self.screen_state = ScreenState::TieGame;
        self.clone()
    }

    pub fn apply_move(&mut self, idx: usize) -> Self {
        self.board[idx] = Some(self.current_player.mark());
        self.switch_player();
        self.clone()
    }

    pub fn switch_player(&mut self) {
        self.current_player = self.current_player.other();
    }
}

#[cfg(test)]
mod spec;
