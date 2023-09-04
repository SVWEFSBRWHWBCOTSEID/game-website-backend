use log::debug;
use crate::models::general::MoveOutcome;
use super::ttt::{PlayerSymbol, col_to_index, check_board_status};


// Validates a c4 move. A move is invalid if:
// 1. It is not in the correct format ("a")
// TODO: check if column is full
pub fn validate_c4_move(new_move: &str) -> bool {
    new_move.chars().nth(0).is_some_and(|c| matches!(c, 'a'..='g'))
}

pub fn process_c4_move(new_move: &str, board: &mut Vec<PlayerSymbol>, is_first: bool) -> MoveOutcome {
    let col = col_to_index(new_move.chars().nth(0).unwrap());
    let index = get_next_unfilled_index(board, col, 7).unwrap();

    board[index] = if is_first {
        PlayerSymbol::First
    } else {
        PlayerSymbol::Second
    };

    // TODO: better move num calc?
    let move_num = board.iter().filter(|m| **m != PlayerSymbol::Empty).count();
    check_board_status(index, move_num, board, 6, 7, 4)
}

fn get_next_unfilled_index(board: &Vec<PlayerSymbol>, column: usize, columns: usize) -> Option<usize> {
    debug!("{} {}", column, columns);
    for i in (column..board.len()).step_by(columns) {
        if board[i] == PlayerSymbol::Empty {
            return Some(i)
        }
    }
    None
}
