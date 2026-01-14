mod human;
mod ui;
mod game;
mod rules;

fn main() {
    let state = game::GameState::new();


    ui::display_board(&state.cells, std::io::stdout());
    ui::prompt_player(state.current_player, std::io::stdout());
}
