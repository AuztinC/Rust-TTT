#[cfg(test)]
mod game_loop {
    use crate::ai;
    use crate::game::*;
    use crate::rules;
    use crate::state::*;
    use std::io::Cursor;

    fn io_helper(input_data: &[u8]) -> (GameState, Cursor<&[u8]>, Vec<u8>) {
        let state = GameState::new();
        let input = Cursor::new(input_data);
        let output: Vec<u8> = Vec::new();
        (state, input, output)
    }

    #[test]
    fn works_through_a_full_game() {
        let (state, mut input, mut output) = io_helper(b"0\n1\n3\n4\n6\n");
        let final_state = game_loop(state, &mut output, &mut input).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert!(final_state._is_game_over());
        assert!(output_str.contains("Player O wins!"));
    }

    #[test]
    fn detects_tie() {
        let (state, mut input, mut output) = io_helper(b"0\n1\n6\n5\n7\n");
        let final_state = game_loop(state, &mut output, &mut input).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert!(final_state._is_tie());
        assert!(output_str.contains("The game is a tie!"));
    }

    #[test]
    fn ai_blocks_and_wins() {
        let mut state = GameState::new();
        state = next_state(&state, 0);
        state = next_state(&state, 4);
        state = next_state(&state, 1);
        let mv = ai::ai_move(&state);
        assert_eq!(mv, 2);
        state = next_state(&state, mv);
        state = next_state(&state, 3);
        let mv = ai::ai_move(&state);
        assert_eq!(mv, 6);
        state = next_state(&state, mv);
        assert_eq!(rules::winner(&state.board()), Some('O'));
    }

    #[test]
    fn ai_forces_tie() {
        let (mut state, _input, output) = io_helper(b"");
        state = state.set_ai_vs_ai_mode();
        let final_state = game_loop(state.clone(), output, _input).unwrap();
        assert!(final_state._is_tie());
    }

    #[test]
    fn ai_always_ties() {
        let state = GameState::new();
        let open_positions = rules::_open_cells(state.board());
        for &pos in open_positions.iter() {
            let (mut state, input, output) = io_helper(b"");
            state = state.set_ai_vs_ai_mode();
            state = state.apply_move(pos);
            state = state.switch_player();

            let final_state = game_loop(state.clone(), output, input).unwrap();
            assert!(final_state._is_tie());
        }
    }
}
