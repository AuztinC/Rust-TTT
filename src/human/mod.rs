use std::io::{self, BufRead, Write};
use crate::ui;
use crate::state::GameState;


fn parse_input(input: &str) -> Result<usize, &'static str> {
    let num: usize = input.trim().parse().map_err(|_| ui::INVALID_CHAR_INPUT)?;
    if num < GameState::BOARD_SIZE {
        Ok(num)
    } else {
        Err(ui::OUT_OF_BOUNDS)
    }
}

pub fn read_move(state: &GameState, input: &mut dyn BufRead, out: &mut dyn Write) -> io::Result<usize> {
    loop {
        let mut line = String::new();
        input.read_line(&mut line)?;

        match parse_input(&line) {
            Ok(pos) => { if state.board()[pos].is_none() {
                 return Ok(pos); 
                } else { 
                    ui::write_error(&mut *out, ui::POSITION_TAKEN)?; 
                } 
            },
            Err(e) => ui::write_error(&mut *out, e)?,
        }
    }
}

#[cfg(test)]
mod spec;
