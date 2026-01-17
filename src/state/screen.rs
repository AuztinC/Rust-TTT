use crate::state::{GameState};
use crate::ui;
use std::io::{self, Write};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ScreenState {
    InGame,
    GameOver,
    TieGame,
}

impl ScreenState {
    pub fn render(&self, state: &GameState, out: &mut dyn Write) -> io::Result<()> {
        match self {
            ScreenState::InGame => {
                ui::display_board(state.board(), &mut *out)?;
                ui::prompt_player(state.current_player().mark(), &mut *out)?;
            }
            ScreenState::GameOver => {
                ui::display_board(state.board(), &mut *out)?;
                ui::announce_winner(state.current_player().mark(), &mut *out)?;
            }
            ScreenState::TieGame => {
                ui::display_board(state.board(), &mut *out)?;
                ui::announce_tie(&mut *out)?;
            }
        }
        out.flush()
    }
}