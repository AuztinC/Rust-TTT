// mod controller;
use crate::state::controller::{Controllers, Controller};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GameMode {
    HumanVsAI,
    AIvsAI,
    HumanVsHuman,
}

impl From<GameMode> for Controllers {
    fn from(m: GameMode) -> Self {
        match m {
            GameMode::HumanVsAI => Controllers { x: Controller::Human, o: Controller::Ai },
            GameMode::HumanVsHuman => Controllers { x: Controller::Human, o: Controller::Human },
            GameMode::AIvsAI => Controllers { x: Controller::Ai, o: Controller::Ai },
        }
    }
}