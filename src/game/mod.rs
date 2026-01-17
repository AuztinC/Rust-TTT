use crate::rules;
use crate::state::{GameState, ScreenState};
use std::io::{self, BufRead, Write};

fn get_move<W: Write, R: BufRead>(state: &GameState, input: &mut R, out: &mut W) -> usize {
    let controller = state.controllers().for_player(state.current_player());
    controller.get_move(state, input, out)
}

pub fn next_state(state: &GameState, mv: usize) -> GameState {
    let mut next = state.clone().apply_move(mv);
    if rules::winner(&next.board()).is_some() {
        return next.set_game_over();
    }
    if rules::is_tie(&next.board()) {
        return next.set_tie();
    }
    next = next.switch_player();
    next
}

pub fn game_loop<W: Write, R: BufRead>(
    mut state: GameState,
    mut out: W,
    mut input: R,
) -> io::Result<GameState> {
    loop {
        state.render_screen(&mut out)?;

        match state.screen_state() {
            ScreenState::InGame => {
                let mv = get_move(&state, &mut input, &mut out);
                state = next_state(&state, mv);
            }

            ScreenState::GameOver | ScreenState::TieGame => {
                return Ok(state);
            }
        }
    }
}

#[cfg(test)]
mod spec;
