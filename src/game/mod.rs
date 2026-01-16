use crate::human;
use crate::ai;
use crate::rules;
use crate::state;
use crate::state::{GameState, Player};
use crate::ui;
use std::io::{self, BufRead, Write};

fn get_move<W: Write, R: BufRead>(state: &GameState,input: &mut R,out: &mut W,) -> usize {
    match state.current_player() {
        Player::X => human::read_move(state, input, out),
        Player::O => ai::ai_move(state),
    }
}

fn next_state(state: &GameState, mv: usize) -> GameState {
    let mut next = state.clone().apply_move(mv);
    if rules::winner(&next.board()).is_some() {
        return next.game_over();
    }
    if rules::is_tie(&next.board()) {
        return next.tie();
    }
    next
}

pub fn game_loop<W: Write, R: BufRead>(mut state: GameState,mut out: W,mut input: R,) -> io::Result<GameState> {
    loop {
        state.render_screen(&mut out)?;

        match state.screen_state() {
            state::ScreenState::InGame => {
                ui::prompt_player(state.current_player().mark(), &mut out)?;
                let mv = get_move(&state, &mut input, &mut out);
                state = next_state(&state, mv);
            }

            state::ScreenState::GameOver | state::ScreenState::TieGame => {
                return Ok(state);
            }
        }
    }
}

#[cfg(test)]
mod spec;
