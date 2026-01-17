#[cfg(test)]
mod game_states {
    use crate::rules::*;
    #[test]
    fn detects_winner_horizontal() {
        let cells = [
            Some('X'),
            Some('X'),
            Some('X'),
            None,
            Some('O'),
            None,
            Some('O'),
            None,
            None,
        ];

        assert_eq!(winner(&cells), Some('X'));
    }

    #[test]
    fn detects_winner_diagonal() {
        let cells = [
            Some('X'),
            Some('O'),
            Some('O'),
            None,
            Some('X'),
            None,
            Some('O'),
            None,
            Some('X'),
        ];

        assert_eq!(winner(&cells), Some('X'));
    }

    #[test]
    fn detects_winner_vertical() {
        let cells = [
            Some('X'),
            None,
            None,
            Some('X'),
            None,
            None,
            Some('X'),
            None,
            None,
        ];

        assert_eq!(winner(&cells), Some('X'));
    }

    #[test]
    fn no_winner_returns_none() {
        let cells = [
            Some('X'),
            Some('O'),
            Some('X'),
            Some('O'),
            Some('X'),
            Some('O'),
            Some('O'),
            Some('X'),
            Some('O'),
        ];
        assert_eq!(winner(&cells), None);
    }

    #[test]
    fn true_when_tie() {
        let cells = [
            Some('X'),
            Some('O'),
            Some('X'),
            Some('O'),
            Some('X'),
            Some('O'),
            Some('O'),
            Some('X'),
            Some('O'),
        ];
        assert_eq!(is_tie(&cells), true);
    }
}
