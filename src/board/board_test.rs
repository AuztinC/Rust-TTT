
#[cfg(test)]
mod tests {
    use crate::board::*;

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
        let cells = 
            [Some('X'), None, Some('O'),
            None, Some('X'), None,
            Some('O'), None, None];
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
        let cells = 
            [Some('X'), Some('O'), Some('X'),
            Some('O'), Some('X'), Some('O'),
            Some('X'), Some('O'), Some('X')];
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
    fn detects_winner() {
        let cells = 
            [Some('X'), Some('X'), Some('X'),
            None, Some('O'), None,
            Some('O'), None, None]; 

        assert_eq!(is_winner(&cells), Some('X'));
    }

    #[test]
    fn no_winner_returns_none() {
        let cells = 
            [Some('X'), Some('O'), Some('X'),
            Some('O'), Some('X'), Some('O'),
            Some('O'), Some('X'), Some('O')];
        assert_eq!(is_winner(&cells), None);
    }

    #[test]
    fn true_when_tie() {
        let cells = 
            [Some('X'), Some('O'), Some('X'),
            Some('O'), Some('X'), Some('O'),
            Some('O'), Some('X'), Some('O')];
        assert_eq!(is_tie(&cells), true);
    }

    #[test]
    fn false_no_tie() {
        let cells = 
            [Some('X'), Some('O'), None,
            Some('O'), Some('X'), Some('O'),
            Some('O'), Some('X'), Some('O')];
        assert_eq!(is_tie(&cells), false);
    }
}