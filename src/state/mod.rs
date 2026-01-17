mod controller;
mod player;
mod game_mode;
mod screen;
pub use player::Player;
pub use game_mode::GameMode;
pub use screen::ScreenState;
use controller::{Controllers};
use std::io::{self, Write};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameState {
    board: [Option<char>; 9],
    current_player: Player,
    screen_state: ScreenState,
    game_mode: GameMode,
    pub(crate) controllers: Controllers,
}

impl GameState {
    pub const BOARD_SIZE: usize = 9;

    pub fn new() -> Self {
        Self {
            board: [None; Self::BOARD_SIZE],
            current_player: Player::X,
            screen_state: ScreenState::InGame,
            game_mode: GameMode::HumanVsAI,
            controllers: Controllers::from(GameMode::HumanVsAI),
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

    pub fn render_screen(&self, out: &mut dyn Write) -> io::Result<()> {
        self.screen_state.render(self, out)
    }

    pub fn game_mode(&self) -> GameMode {
        self.game_mode.clone()
    }

    pub fn controllers(&self) -> Controllers {
        self.controllers.clone()
    }

    pub fn set_ai_vs_ai_mode(&mut self) -> Self {
        self.game_mode = GameMode::AIvsAI;
        self.controllers = Controllers::from(self.game_mode.clone());
        self.clone()
    }

    pub fn set_hvh_mode(&mut self) -> Self {
        self.game_mode = GameMode::HumanVsHuman;
        self.controllers = Controllers::from(self.game_mode.clone());
        self.clone()
    }

    pub fn _is_game_over(&self) -> bool {
        matches!(self.screen_state, ScreenState::GameOver)
    }

    pub fn _is_tie(&self) -> bool {
        matches!(self.screen_state, ScreenState::TieGame)
    }

    pub fn set_game_over(&mut self) -> Self {
        self.screen_state = ScreenState::GameOver;
        self.clone()
    }

    pub fn set_tie(&mut self) -> Self {
        self.screen_state = ScreenState::TieGame;
        self.clone()
    }

    pub fn apply_move(&mut self, idx: usize) -> Self {
        self.board[idx] = Some(self.current_player.mark());
        self.clone()
    }

    pub fn switch_player(&mut self) -> Self {
        self.current_player = self.current_player.other();
        self.clone()
    }
}

#[cfg(test)]
mod spec;
