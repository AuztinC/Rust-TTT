#[cfg(test)]
mod input {

    use crate::human::*;
    use crate::state;
    use crate::ui::{INVALID_CHAR_INPUT, OUT_OF_BOUNDS, POSITION_TAKEN};
    use std::io::Cursor;

    fn io_helper(input_data: &[u8]) -> (usize, Vec<u8>) {
        let state = state::GameState::new();
        let mut input = Cursor::new(input_data);
        let mut output: Vec<u8> = Vec::new();
        let pos = read_move(&state, &mut input, &mut output);
        (pos, output)
    }

    #[test]
    fn returns_valid_position() {
        let (pos, out) = io_helper(b"4\n");
        assert_eq!(pos, 4);
        assert!(out.is_empty());
    }

    #[test]
    fn retries_until_valid_and_writes_error() {
        let (pos, out) = io_helper(b"a\n4\n");
        let printed = String::from_utf8(out).unwrap();
        assert!(printed.contains(INVALID_CHAR_INPUT));
        assert_eq!(pos, 4);
    }

    #[test]
    fn retries_on_out_of_bounds() {
        let (pos, out) = io_helper(b"9\n0\n");
        let printed = String::from_utf8(out).unwrap();
        assert_eq!(pos, 0);
        assert!(printed.contains(OUT_OF_BOUNDS));
    }

    #[test]
    fn retries_on_position_taken() {
        let mut state = state::GameState::new();
        state.apply_move(0);
        let mut input = Cursor::new(b"0\n1\n");
        let mut output: Vec<u8> = Vec::new();
        let pos = read_move(&state, &mut input, &mut output);
        let printed = String::from_utf8(output).unwrap();
        assert_eq!(pos, 1);
        assert!(printed.contains(POSITION_TAKEN));
    }
}
