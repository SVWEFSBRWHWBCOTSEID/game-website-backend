use super::ttt::{PlayerSymbol, check_board_status, row_to_index, col_to_index};
use crate::models::general::MoveOutcome;


// Validates a uttt move. A move is invalid if:
// 1. It is not in the correct format ("a1b2")
// 2. It has already been played
// 3. The active board is not -1 (any open board) and it was not placed in the currently active board
// 4. The active board is -1 and it was placed in an already full board
pub fn validate_uttt_move(new_move: &str, moves: Vec<&str>, game_states: &Vec<MoveOutcome>, active_board: i32) -> bool {
    if moves.contains(&new_move)
        || !new_move.chars().nth(0).is_some_and(|c| matches!(c, 'a'..='c'))
        || !new_move.chars().nth(1).is_some_and(|c| c.is_digit(10))
        || !new_move.chars().nth(2).is_some_and(|c| matches!(c, 'a'..='c'))
        || !new_move.chars().nth(3).is_some_and(|c| c.is_digit(10)) {
        return false;
    }

    let outer = col_to_index(new_move.chars().nth(0).unwrap())
        + row_to_index(new_move.chars().nth(1).unwrap()) * 3;

    if active_board == -1 {
        game_states[outer] == MoveOutcome::None
    } else {
        outer == active_board as usize
    }
}

pub fn process_uttt_move(
    new_move: &str,
    board: &mut Vec<Vec<PlayerSymbol>>,
    board_states: &mut Vec<MoveOutcome>,
    active_board: &mut i32,
    is_first: bool
) -> MoveOutcome {
    let outer = col_to_index(new_move.chars().nth(0).unwrap())
        + row_to_index(new_move.chars().nth(1).unwrap()) * 3;

    let inner = col_to_index(new_move.chars().nth(2).unwrap())
        + row_to_index(new_move.chars().nth(3).unwrap()) * 3;

    // 1. Set the square on the inner board to the given player symbol.
    board[outer][inner] = if is_first {
        PlayerSymbol::First
    } else {
        PlayerSymbol::Second
    };

    // 2. Update the inner board status by running the ttt board check function on it.
    // Do this before updating the active board in case the move points back to the
    // same square and simultaneously wins that square.
    // TODO: better move num calc?
    let move_num = board[outer].iter().filter(|m| **m != PlayerSymbol::Empty).count();
    board_states[outer] = check_board_status(inner, move_num, &board[outer], 3, 3, 3);

    // 3. Finally, update the active board.
    *active_board = if board_states[inner] != MoveOutcome::None {
        -1
    } else {
        inner as i32
    };

    // Map game state vec to ttt symbols to check the status of the outer board
    let outer_move_num = board_states.iter().filter(|m| **m != MoveOutcome::None).count();
    let outer_board = board_states.iter().map(|m| match m {
        MoveOutcome::FirstWin => PlayerSymbol::First,
        MoveOutcome::SecondWin => PlayerSymbol::Second,
        _ => PlayerSymbol::Empty
    }).collect::<Vec<_>>();

    check_board_status(outer, outer_move_num, &outer_board, 3, 3, 3)
}
