#[cfg(test)]
mod main {

    use crate::initial_state;
    use crate::state::{ GameMode, GameState };

    fn io_helper(args: &Option<String>) -> (String, GameState) {
        let mut out = Vec::new();
        let state = initial_state(args, &mut out);
        let printed = String::from_utf8(out).unwrap();
        (printed, state)
    }

    #[test]
    fn default_human_vs_ai_mode() {
        let (printed, state) = io_helper(&None);
        assert!(printed.contains("Welcome to Tic Tac Toe! \n Player X goes first. Choose a square by entering a number from 0 to 8."));
        assert_eq!(state.game_mode(), GameMode::HumanVsAI);
    }

    #[test]
    fn can_set_ai_vs_ai_mode() {
        let (printed, state) = io_helper(&Some("-AIvAI".to_string()));
        assert!(printed.contains("Starting game: -AIvAI"));
        assert_eq!(state.game_mode(), GameMode::AIvsAI);
    }

    #[test]
    fn can_set_human_vs_human_mode() {
        let (printed, state) = io_helper(&Some("-HvH".to_string()));
        assert!(printed.contains("Starting game: -HvH"));
        assert_eq!(state.game_mode(), GameMode::HumanVsHuman);
    }

    #[test]
    fn prompt_for_bad_argument() {
        let (printed, state) = io_helper(&Some("-Blah".to_string()));
        assert!(printed.contains("Unknown argument: '-Blah' Defaulting to Human vs AI mode. \n Use '-- -HvH' to play Human vs Human or '-- -AIvAI' to watch AI vs AI."));
        assert_eq!(state.game_mode(), GameMode::HumanVsAI);
    }
}
