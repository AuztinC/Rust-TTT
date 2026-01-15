use crate::state::GameState;

mod state;
mod game;
mod human;
mod rules;
mod ui;


fn main() -> std::io::Result<()> {
    let initial_state = GameState::new();
    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    let stdout = std::io::stdout();
    let output = stdout.lock();

    game::game_loop(initial_state, output, &mut input)?;
    Ok(())
}
