#[cfg(test)]
mod game_loop {
    use crate::game::*;
    use crate::state::GameState;
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
        assert!(final_state.is_game_over());
        assert!(output_str.contains("Player X wins!"));
    }

    #[test]
    fn detects_tie() {
        let (state, mut input, mut output) = io_helper(b"0\n1\n2\n4\n3\n5\n7\n6\n8\n");
        let final_state = game_loop(state, &mut output, &mut input).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert!(final_state.is_tie());
        assert!(output_str.contains("The game is a tie!"));
    }

}
