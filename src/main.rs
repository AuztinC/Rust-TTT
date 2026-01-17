use crate::state::GameState;

mod ai;
mod game;
mod human;
mod rules;
mod state;
mod ui;

pub fn initial_state(args: &Option<String>) -> GameState {
    let mut state = GameState::new();
    if args == &Some("-AIvAI".to_string()) {
        state = state.set_ai_vs_ai_mode()
    } else if args == &Some("-HvH".to_string()) {
        state = state.set_hvh_mode()
    }
    state
}

fn main() -> std::io::Result<()> {
    let args = std::env::args().nth(1);
    let initial_state = initial_state(&args);
    let stdin = std::io::stdin();
    let input = stdin.lock();
    let stdout = std::io::stdout();
    let output = stdout.lock();
    if args.is_some() {
        println!("Starting game: {}", args.unwrap());
    }

    game::game_loop(initial_state, output, input)?;
    Ok(())
}

#[cfg(test)]
mod main_spec;
