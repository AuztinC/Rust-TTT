#[cfg(test)]
mod main {

    use crate::initial_state;
    use crate::state::GameMode;

    #[test]
    fn default_human_vs_ai_mode() {
        let state = initial_state(&Some("None".to_string()));
        assert_eq!(state.game_mode(), GameMode::HumanVsAI);
    }

    #[test]
    fn can_set_ai_vs_ai_mode() {
        let state = initial_state(&Some("-AIvAI".to_string()));
        assert_eq!(state.game_mode(), GameMode::AIvsAI);
    }

    #[test]
    fn can_set_human_vs_human_mode() {
        let state = initial_state(&Some("-HvH".to_string()));
        assert_eq!(state.game_mode(), GameMode::HumanVsHuman);
    }
}
