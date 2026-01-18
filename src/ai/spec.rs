#[cfg(test)]
mod ai {
    use crate::ai::*;
    use crate::game::next_state;
    use crate::state::GameState;

    fn is_game(state: &GameState) -> bool {
        rules::winner(&state.board()).is_none() && !rules::is_tie(&state.board())
    }

    #[test]
    fn center_on_empty_board() {
        let state = GameState::new();
        let mv = ai_move(&state);
        assert_eq!(mv, 4);
    }

    #[test]
    fn corner_when_center_taken() {
        let mut state = GameState::new();
        state = next_state(&state, 4);
        let mv = ai_move(&state);
        assert!([0, 2, 6, 8].contains(&mv));
    }

    #[test]
    fn block_opponent_win() {
        let mut state = GameState::new();
        state = next_state(&state, 0);
        state = next_state(&state, 4);
        state = next_state(&state, 1);
        let mv = ai_move(&state);
        assert_eq!(mv, 2);
    }

    #[test]
    fn win_if_possible() {
        let mut state = GameState::new();
        state = next_state(&state, 0);
        state = next_state(&state, 4);
        state = next_state(&state, 8);
        state = next_state(&state, 1);
        state = next_state(&state, 3);
        let mv = ai_move(&state);
        assert_eq!(mv, 7);
    }

    #[test]
    fn prefer_win_over_block() {
        let mut state = GameState::new();
        state = next_state(&state, 0);
        state = next_state(&state, 4);
        state = next_state(&state, 3);
        state = next_state(&state, 1);
        state = next_state(&state, 5);
        let mv = ai_move(&state);
        assert_eq!(mv, 7);
    }

    #[test]
    fn force_tie_when_cannot_win() {
        let mut state = GameState::new();
        state = next_state(&state, 0);
        state = next_state(&state, 4);
        state = next_state(&state, 1);
        state = next_state(&state, 2);
        state = next_state(&state, 7);
        let mv = ai_move(&state);
        assert_eq!(mv, 6);
    }

    #[test]
    fn ai_from_empty_board_is_tie() {
        let mut state = GameState::new();

        while is_game(&state) {
            let mv = ai_move(&state);
            state = next_state(&state, mv);
        }

        assert!(
            rules::winner(&state.board()).is_none(),
            "winner found:\n{:?}",
            state.board()
        );
        assert!(
            rules::is_tie(&state.board()),
            "not a tie:\n{:?}",
            state.board()
        );
    }

    #[test]
    fn ai_ties_from_any_first_move() {
        let state: GameState = GameState::new();
        let open_positions = rules::_open_cells(state.board());
        for pos in open_positions {
            let mut state = GameState::new();
            state = next_state(&state, pos);

            while is_game(&state) {
                let mv = ai_move(&state);
                state = next_state(&state, mv);
            }

            assert!(
                rules::winner(&state.board()).is_none() && rules::is_tie(&state.board()),
                "failed starting at {pos}, final board: {:?}",
                state.board()
            );
        }
    }
}
