#[cfg(test)]
mod state {
    use crate::state::*;
    use crate::ui;

    fn io_helper() -> (GameState, Vec<u8>) {
        let state = GameState::new();
        let output: Vec<u8> = Vec::new();
        (state, output)
    }

    #[test]
    fn new_game_starts_with_empty_board_and_x_player() {
        let state = GameState::new();
        assert_eq!(state.board(), &[None; 9]);
        assert_eq!(state.current_player(), Player::X);
        assert_eq!(state.screen_state, ScreenState::InGame);
        assert_eq!(state._is_game_over(), false);
    }

    #[test]
    fn get_current_player() {
        let state = GameState::new();
        assert_eq!(state.current_player(), Player::X);
    }

    #[test]
    fn get_board() {
        let state = GameState::new();
        assert_eq!(state.board(), &[None; 9]);
    }

    #[test]
    fn switching_player() {
        let mut state = GameState::new();
        assert_eq!(state.current_player, Player::X);
        state = state.switch_player();
        assert_eq!(state.current_player, Player::O);
        state = state.switch_player();
        assert_eq!(state.current_player, Player::X);
    }

    #[test]
    fn applying_move_updates_board() {
        let mut state = GameState::new();
        let _ = state.apply_move(0);
        state = state.switch_player();
        assert_eq!(state.board[0], Some('X'));
        let _ = state.apply_move(1);
        state = state.switch_player();
        assert_eq!(state.board[1], Some('O'));
    }

    #[test]
    fn game_over() {
        let mut state = GameState::new();
        state = state.set_game_over();
        assert_eq!(state._is_game_over(), true);
    }

    #[test]
    fn as_tie() {
        let mut state = GameState::new();
        state = state.set_tie();
        assert_eq!(state._is_tie(), true);
    }

    #[test]
    fn render_screen_in_game() {
        let (state, mut output) = io_helper();
        let mut prompt_output = Vec::new();
        state.render_screen(&mut output).unwrap();
        ui::prompt_player('X', &mut prompt_output).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        let prompt_str = String::from_utf8(prompt_output).unwrap();
        assert!(output_str.contains(prompt_str.trim()));
    }

    #[test]
    fn render_screen_game_over() {
        let (mut state, mut output) = io_helper();
        state = state.set_game_over();
        state.render_screen(&mut output).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("Player"));
        assert!(output_str.contains("wins!"));
    }

    #[test]
    fn render_screen_tie_game() {
        let (mut state, mut output) = io_helper();
        state = state.set_tie();
        state.render_screen(&mut output).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("The game is a tie!"));
    }

    #[test]
    fn human_vs_ai_mode() {
        let state = GameState::new();
        assert_eq!(state.game_mode(), GameMode::HumanVsAI);
    }

    #[test]
    fn ai_vs_ai_mode() {
        let mut state = GameState::new();
        state.game_mode = GameMode::AIvsAI;
        assert_eq!(state.game_mode(), GameMode::AIvsAI);
    }
}
