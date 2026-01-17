use crate::ai;
use crate::human;
use crate::state::player::Player;
use crate::state::{GameState};
use std::io::{BufRead, Write};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Controller {
    Human,
    Ai,
}

impl Controller {
    pub fn get_move<R: BufRead, W: Write>(
        self,
        state: &GameState,
        input: &mut R,
        out: &mut W,
    ) -> usize {
        match self {
            Controller::Human => human::read_move(state, input, out),
            Controller::Ai => ai::ai_move(state),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Controllers {
    pub x: Controller,
    pub o: Controller,
}

impl Controllers {
    pub fn for_player(self, p: Player) -> Controller {
        match p {
            Player::X => self.x,
            Player::O => self.o,
        }
    }
}
