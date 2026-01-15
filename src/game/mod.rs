use crate::human;
use crate::rules;
use crate::state::GameState;
use crate::state::Player;
use crate::ui;
use std::io::{self, Write};

fn get_move<W: Write, R: io::BufRead>(
    state: &GameState,
    input: &mut R,
    out: &mut W,
) -> io::Result<usize> {
    match state.current_player() {
        Player::X => human::read_move(&state, input, out),
        Player::O => human::read_move(&state, input, out),
    }
}

pub fn game_loop<W: Write, R: io::BufRead>(
    mut state: GameState,
    mut out: W,
    input: &mut R,
) -> io::Result<GameState> {
    loop {
        state.render_screen(&mut out)?;

        if let Some(winner) = rules::winner(&state.board()) {
            // ui::announce_winner(winner, &mut out)?;
            // out.flush()?;
            state.game_over();
            state.render_screen(&mut out)?;
            return Ok(state);
        }
        
        if rules::is_tie(&state.board()) {
            // ui::announce_tie(&mut out)?;
            state.tie_game();
            state.render_screen(&mut out)?;
            return Ok(state);
        }

        ui::prompt_player(state.current_player().mark(), &mut out)?;
        let mv = get_move(&state, input, &mut out)?;
        state.next_state(mv);
    }
}

#[cfg(test)]
mod spec;
