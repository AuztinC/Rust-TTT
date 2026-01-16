#[cfg(test)]
mod ai {
    use crate::ai::*;
    use crate::state::{GameState};

    #[test]
    fn center_on_empty_board() {
        let state = GameState::new();
        let mv = ai_move(&state);
        assert_eq!(mv, 4);
    }

    #[test]
    fn corner_when_center_taken() {
        let mut state = GameState::new();
        state.apply_move(4);
        let mv = ai_move(&state);
        assert!([0, 2, 6, 8].contains(&mv));
    }

    #[test]
    fn block_opponent_win() {
        let mut state = GameState::new();
        state.apply_move(0);
        state.apply_move(4);
        state.apply_move(1);
        let mv = ai_move(&state);
        assert_eq!(mv, 2);
    }

    #[test]
    fn win_if_possible() {
        let mut state = GameState::new();
        state.apply_move(0);
        state.apply_move(4);
        state.apply_move(8);
        state.apply_move(1);
        state.apply_move(3);
        let mv = ai_move(&state);
        assert_eq!(mv, 7);
    }

    #[test]
    fn prefer_win_over_block() {
        let mut state = GameState::new();
        state.apply_move(0);
        state.apply_move(4);
        state.apply_move(3);
        state.apply_move(1);
        state.apply_move(5);
        let mv = ai_move(&state);
        assert_eq!(mv, 7);
    }

    #[test]
    fn force_tie_when_cannot_win() {
        let mut state = GameState::new();
        state.apply_move(0);
        state.apply_move(4);
        state.apply_move(1);
        state.apply_move(2);
        state.apply_move(7);
        let mv = ai_move(&state);
        assert_eq!(mv, 6);
    }

}