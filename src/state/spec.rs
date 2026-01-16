#[cfg(test)]
mod state {
    use crate::state::*;

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
        state.switch_player();
        assert_eq!(state.current_player, Player::O);
        state.switch_player();
        assert_eq!(state.current_player, Player::X);
    }

    #[test]
    fn applying_move_updates_board() {
        let mut state = GameState::new();
        let _ = state.apply_move(0);
        assert_eq!(state.board[0], Some('X'));
        let _ = state.apply_move(1);
        assert_eq!(state.board[1], Some('O'));
    }

    #[test]
    fn game_over() {
        let mut state = GameState::new();
        state.game_over();
        assert_eq!(state._is_game_over(), true);
    }

    #[test]
    fn as_tie() {
        let mut state = GameState::new();
        state.tie();
        assert_eq!(state._is_tie(), true);
    }
}
