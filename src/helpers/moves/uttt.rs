pub fn validate_uttt_move(moves: Vec<&str>, new_move: &str) -> bool {
    !moves.contains(&new_move)
        && new_move.chars().nth(0).is_some_and(|c| matches!(c, 'a'..='c'))
        && new_move.chars().nth(1).is_some_and(|c| c.is_digit(10))
        && new_move.chars().nth(2).is_some_and(|c| matches!(c, 'a'..='c'))
        && new_move.chars().nth(3).is_some_and(|c| c.is_digit(10))

    // TODO: active board
}
