#[cfg(test)]
mod board_rendering {
    use crate::ui::*;
    #[test]
    fn empty_board_with_indices() {
        let cells = [None; 9];

        let expected = "  \
  0 | 1 | 2
 -----------
  3 | 4 | 5
 -----------
  6 | 7 | 8
-------------
";

        assert_eq!(render_board(&cells), expected);
    }

    #[test]
    fn partially_filled_board() {
        let cells = [
            Some('X'),
            None,
            Some('O'),
            None,
            Some('X'),
            None,
            Some('O'),
            None,
            None,
        ];
        let expected = "  \
  X | 1 | O
 -----------
  3 | X | 5
 -----------
  O | 7 | 8
-------------
";

        assert_eq!(render_board(&cells), expected);
    }

    #[test]
    fn filled_board() {
        let cells = [
            Some('X'),
            Some('O'),
            Some('X'),
            Some('O'),
            Some('X'),
            Some('O'),
            Some('X'),
            Some('O'),
            Some('X'),
        ];
        let expected = "  \
  X | O | X
 -----------
  O | X | O
 -----------
  X | O | X
-------------
";

        assert_eq!(render_board(&cells), expected);
    }

    #[test]
    fn display_board_writes_to_output() {
        let cells = [
            Some('X'),
            None,
            Some('O'),
            None,
            Some('X'),
            None,
            Some('O'),
            None,
            None,
        ];

        let expected_output = "  \
  X | 1 | O
 -----------
  3 | X | 5
 -----------
  O | 7 | 8
-------------
";

        let mut output = Vec::new();
        let _ = display_board(&cells, &mut output);
        let output_str = String::from_utf8(output).unwrap();

        assert_eq!(output_str, expected_output);
    }
}

#[cfg(test)]
mod response_messages {
    use crate::ui::*;

    use std::io::{self, Write};

    fn with_output<F>(f: F) -> String
    where
        F: FnOnce(&mut dyn Write) -> io::Result<()>,
    {
        let mut buf = Vec::<u8>::new();
        f(&mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }

    #[test]
    fn print_error() {
        let error_msg = "Some error occurred.";
        let out = with_output(|w| write_error(w, error_msg));
        assert_eq!(out, "Some error occurred.\n");
    }

    #[test]
    fn input_out_of_bounds() {
        let out = with_output(|w| write_error(w, OUT_OF_BOUNDS));
        assert_eq!(
            out,
            "Invalid input. Please enter a number between 0 and 8.\n"
        );
    }

    #[test]
    fn position_taken() {
        let out = with_output(|w| write_error(w, POSITION_TAKEN));
        assert_eq!(out, "Position already taken. Please choose another.\n");
    }

    #[test]
    fn invalid_char() {
        let out = with_output(|w| write_error(w, INVALID_CHAR_INPUT));
        assert_eq!(out, "Please enter a number\n");
    }

    #[test]
    fn prompt_player_print() {
        let player = 'X';
        let out = with_output(|w| prompt_player(player, w));
        assert_eq!(out, "Player X, enter your move: \n");
    }

    #[test]
    fn announce_winner_print() {
        let winner = 'O';
        let out = with_output(|w| announce_winner(winner, w));
        assert_eq!(out, "Player O wins!\n");
    }

    #[test]
    fn announce_tie_print() {
        let out = with_output(|w| announce_tie(w));
        assert_eq!(out, "The game is a tie!\n");
    }
}
