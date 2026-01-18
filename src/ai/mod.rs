use crate::rules;
use crate::state::{GameState, Player};

fn terminal_score(state: &GameState, maximizer: Player, depth: i32) -> Option<i32> {
    if let Some(winner) = rules::winner(&state.board()) {
        if winner == maximizer.mark() {
            return Some(10 - depth);
        } else {
            return Some(depth - 10);
        }
    }
    if rules::is_tie(&state.board()) {
        return Some(0);
    }
    None
}

fn minimax(
    state: &GameState,
    maximizer: Player,
    depth: i32,
    mut alpha: i32,
    mut beta: i32,
) -> i32 {
    if let Some(score) = terminal_score(state, maximizer, depth) {
        return score;
    }
    let is_maximizing = state.current_player() == maximizer;
    let mut best = if is_maximizing { i32::MIN } else { i32::MAX };

    for (i, spot) in state.board().iter().enumerate() {
        if spot.is_none() {
            let mut next = state.clone();
            next = next.apply_move(i);
            next = next.switch_player();

            let score = minimax(
                &next,
                maximizer,
                depth + 1,
                alpha,
                beta,
            );

            if is_maximizing {
                best = best.max(score);
                alpha = alpha.max(best);
            } else {
                best = best.min(score);
                beta = beta.min(best);
            }
            if beta <= alpha {
                break;
            }
        }
    }
    best
}

fn hard(state: &GameState) -> usize {
    let maximizer = state.current_player();

    let mut best_move: usize = 0;
    let mut best_score = i32::MIN;
    let priority = [4, 0, 2, 6, 8, 1, 3, 5, 7];

    for &i in priority.iter() {
        if state.board()[i].is_none() {
            let mut next = state.clone();
            next = next.apply_move(i);
            next = next.switch_player();

            let score = minimax(
                &next,
                maximizer,
                1,
                i32::MIN,
                i32::MAX,
            );

            if score > best_score {
                best_score = score;
                best_move = i;
            }
        }
    }
    best_move
}

pub fn ai_move(state: &GameState) -> usize {
    let mv = hard(state);

    mv
}

#[cfg(test)]
mod spec;
