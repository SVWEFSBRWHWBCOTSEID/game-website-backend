use parking_lot::{MutexGuard};

use crate::{prisma::game, models::general::MoveOutcome};
use crate::helpers::moves::ttt::{col_to_index, row_to_index, TTTSymbol};
use crate::helpers::moves::uttt::validate_uttt_move;
use crate::lumber_mill::GameBoard::{TTTBoard, UTTTBoard};
use crate::lumber_mill::LumberMill;
use super::ttt::{validate_ttt_move, check_ttt_board_status};


impl game::Data {
    // check if new move is legal
    pub fn validate_new_move(&self, new_move: &str) -> bool {
        match self.game_key.as_str() {
            "ttt" => validate_ttt_move(self.get_moves_vec_str(), &new_move),
            "uttt" => validate_uttt_move(self.get_moves_vec_str(), &new_move),
            "c4" => false,
            "pc" => false,
            _ => false,
        }
    }

    // Updates the game board with the given move, checking the new board for victories and
    // returning the resultant `MoveOutcome`.
    pub fn update_and_check(&self, new_move: &str, mut mill: MutexGuard<LumberMill>, is_first: bool) -> MoveOutcome {
        match mill.boards.get_mut(&self.id.clone()).unwrap() {
            TTTBoard(board) => {
                let m = col_to_index(new_move.chars().nth(0).unwrap())
                    + row_to_index(new_move.chars().nth(1).unwrap()) * 3;

                board[m] = if is_first {
                    TTTSymbol::X
                } else {
                    TTTSymbol::O
                };

                check_ttt_board_status(m, self.get_moves_vec_str().len(), board, 3, 3, 3)
            },
            UTTTBoard(board, active_board, board_states) => {
                let outer = col_to_index(new_move.chars().nth(0).unwrap())
                    + row_to_index(new_move.chars().nth(1).unwrap()) * 3;

                let inner = col_to_index(new_move.chars().nth(2).unwrap())
                    + row_to_index(new_move.chars().nth(3).unwrap()) * 3;

                // 1. Set the square on the inner board to the given player symbol.
                board[outer][inner] = if is_first {
                    TTTSymbol::X
                } else {
                    TTTSymbol::O
                };

                // 2. Update the inner board status by running the ttt board check function on it.
                // Do this before updating the active board in case the move points back to the
                // same square and simultaneously wins that square.
                // TODO: better move num calc?
                let move_num = board[outer].iter().filter(|m| **m != TTTSymbol::Empty).count();
                board_states[outer] = check_ttt_board_status(inner, move_num, &board[outer], 3, 3, 3);

                // 3. Finally, update the active board.
                *active_board = if board_states[inner] != MoveOutcome::None {
                    -1
                } else {
                    inner as i32
                };

                // Map game state vec to ttt symbols to check the status of the outer board
                let outer_move_num = board_states.iter().filter(|m| **m != MoveOutcome::None).count();
                let outer_board = board_states.iter().map(|m| match m {
                    MoveOutcome::FirstWin => TTTSymbol::X,
                    MoveOutcome::SecondWin => TTTSymbol::O,
                    _ => TTTSymbol::Empty
                }).collect::<Vec<_>>();

                check_ttt_board_status(outer, outer_move_num, &outer_board, 3, 3, 3)
            },
            // "c4" => MoveOutcome::None,
            // "pc" => MoveOutcome::None,
        }
    }
}
