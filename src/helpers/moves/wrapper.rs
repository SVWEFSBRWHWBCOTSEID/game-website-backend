use crate::{prisma::game, models::general::MoveOutcome};
use crate::helpers::moves::ttt::{col_to_index, row_to_index, TTTSymbol};
use super::ttt::{validate_ttt_move, check_board_status};


impl game::Data {
    // check if new move is legal
    pub fn validate_new_move(&self, new_move: &str) -> bool {
        match self.game_key.as_str() {
            "ttt" => validate_ttt_move(self.get_moves_vec_str(), &new_move),
            "uttt" => false,
            "c4" => false,
            "pc" => false,
            _ => false,
        }
    }

    // Updates the game board with the given move, checking the new board for victories and
    // returning the `MoveOutcome`.
    pub fn update_and_check(&self, new_move: &str, board: &mut Vec<TTTSymbol>, is_first: bool) -> MoveOutcome {
        match self.game_key.as_str() {
            "ttt" => {
                let m = col_to_index(new_move.chars().nth(0).unwrap())
                    + row_to_index(new_move.chars().nth(1).unwrap()) * 3;

                board[m] = if is_first {
                    TTTSymbol::X
                } else {
                    TTTSymbol::O
                };

                check_board_status(m, self.get_moves_vec_str().len(), board, 3, 3, 3)
            },
            "uttt" => MoveOutcome::None,
            "c4" => MoveOutcome::None,
            "pc" => MoveOutcome::None,
            _ => MoveOutcome::None,
        }
    }
}
