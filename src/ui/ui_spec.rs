#[cfg(test)]
mod tests {
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
    fn prints_empty_board_with_indices() {
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
    fn prints_partially_filled_board() {
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
    fn prints_filled_board() {
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
        display_board(&cells, &mut output);
        let output_str = String::from_utf8(output).unwrap();

        assert_eq!(output_str, expected_output);
    }

    #[test]
    fn input_out_of_bounds_prints_message() {
        let out = with_output(|w| Ok(input_out_of_bounds(w)));
        assert_eq!(
            out,
            "Invalid input. Please enter a number between 0 and 8.\n"
        );
    }

    #[test]
    fn prompt_player_prints_message() {
        let player = 'X';
        let out: String = with_output(|w| Ok(prompt_player(player, w)));
        assert_eq!(out, "Player X, enter your move (0-8): \n");
    }
}
