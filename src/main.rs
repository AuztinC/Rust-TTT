use crate::state::GameState;

mod ai;
mod game;
mod human;
mod rules;
mod state;
mod ui;

pub fn initial_state(args: &Option<String>, mut out: impl std::io::Write) -> GameState {
    let mut state = GameState::new();
    if args == &Some("-AIvAI".to_string()) {
        state = state.set_ai_vs_ai_mode()
    } else if args == &Some("-HvH".to_string()) {
        state = state.set_hvh_mode()
    } else {
        if args.is_some() {
            ui::unknown_argument(args.as_ref().unwrap(), &mut out).unwrap();
        }
    }

    if args.is_some() {
        if args.clone().unwrap() == "-AIvAI" || args.clone().unwrap() == "-HvH" {
            ui::starting_game(args.as_ref().unwrap(), &mut out).unwrap();
        } else {
            ui::starting_game("Human vs AI", &mut out).unwrap();
        }
    }
    ui::welcome(&mut out).unwrap();
    state
}

fn main() -> std::io::Result<()> {
    let args = std::env::args().nth(1);
    let stdin = std::io::stdin();
    let input = stdin.lock();
    let stdout = std::io::stdout();
    let mut output = stdout.lock();
    let initial_state = initial_state(&args, &mut output);

    game::game_loop(initial_state, output, input)?;
    Ok(())
}

#[cfg(test)]
mod main_spec;
