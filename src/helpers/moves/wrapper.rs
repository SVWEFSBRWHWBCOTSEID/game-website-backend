use crate::{prisma::game, models::general::MoveOutcome};
use crate::helpers::moves::ttt::{str_to_move_num, TTTSymbol};
use super::ttt::{validate_ttt_move, ttt_move_outcome};


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
                let m = str_to_move_num(new_move, 3);
                board[m] = if is_first {
                    TTTSymbol::X
                } else {
                    TTTSymbol::O
                };
                ttt_move_outcome(&new_move, self.get_moves_vec_str().len(), board, 3, 3, 3)
            },
            "uttt" => MoveOutcome::None,
            "c4" => MoveOutcome::None,
            "pc" => MoveOutcome::None,
            _ => MoveOutcome::None,
        }
    }
}
