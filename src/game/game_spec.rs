#[cfg(test)]
mod tests {
    use crate::game::*;
    use crate::ui::position_taken;

    #[test]
    fn new_game_starts_with_empty_board_and_x_player() {
        let state = GameState::new();
        assert_eq!(state.cells, [None; 9]);
        assert_eq!(state.current_player, 'X');
        assert_eq!(state.game_over, false);
    }

    #[test]
    fn switching_player_changes_current_player() {
        let mut state = GameState::new();
        assert_eq!(state.current_player, 'X');
        state.switch_player();
        assert_eq!(state.current_player, 'O');
        state.switch_player();
        assert_eq!(state.current_player, 'X');
    }

    #[test]
    fn applying_move_updates_board() {
        let mut state = GameState::new();
        state.maybe_apply_move(0, || {});
        assert_eq!(state.cells[0], Some('X'));
        state.switch_player();
        state.maybe_apply_move(1, || {});
        assert_eq!(state.cells[1], Some('O'));
    }

    #[test]
    fn applying_move_to_occupied_cell_does_not_change_board() {
        let mut state = GameState::new();
        let mut buf: Vec<u8> = Vec::new();
        state.maybe_apply_move(0, || position_taken(&mut buf));
        state.switch_player();
        state.maybe_apply_move(0, || position_taken(&mut buf));
        let out = String::from_utf8(buf).unwrap();
        assert_eq!(out, "Position already taken. Please choose another.\n");
        assert_eq!(state.cells[0], Some('X'));
    }
}
