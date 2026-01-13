

#[cfg(test)]
mod tests {
    use super::*;

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



pub fn render_board(cells: &[Option<char>; 9]) -> String {
    fn cell_text(cells: &[Option<char>; 9], idx: usize) -> String {
        match cells[idx] {
            Some(ch) => format!("{}" , ch),
            None => format!("{}", idx),
        }
    }

    let rows = [
        (0, 1, 2),
        (3, 4, 5),
        (6, 7, 8),
    ];

    let mut out = String::new();

    for (row, (a, b, c)) in rows.iter().enumerate() {
        out.push_str(&format!(
            "  {} | {} | {}\n",
            cell_text(cells, *a),
            cell_text(cells, *b),
            cell_text(cells, *c),
        ));

        if row < 2 {
            out.push_str(" -----------\n");
        }
    }

    out.push_str("-------------\n");
    out
}

pub fn display_board(cells: &[Option<char>; 9]) {
    print!("{}", render_board(cells));
}

pub fn is_winner(cells: &[Option<char>; 9]) -> Option<char> {
    let winning_combinations = [
        (0, 1, 2),
        (3, 4, 5),
        (6, 7, 8),
        (0, 3, 6),
        (1, 4, 7),
        (2, 5, 8),
        (0, 4, 8),
        (2, 4, 6),
    ];

    for &(a, b, c) in &winning_combinations {
        if let (Some(ch1), Some(ch2), Some(ch3)) = (cells[a], cells[b], cells[c]) {
            if ch1 == ch2 && ch2 == ch3 {
                return Some(ch1);
            }
        }
    }

    None
}

pub fn is_tie(cells: &[Option<char>; 9]) -> bool {
    cells.iter().all(|cell| cell.is_some())
}
