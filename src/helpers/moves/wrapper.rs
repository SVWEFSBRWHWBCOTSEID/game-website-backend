use crate::{prisma::game, models::general::MoveOutcome};
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

    // get the outcome of next move
    pub fn new_move_outcome(&self, new_move: &str) -> MoveOutcome {
        match self.game_key.as_str() {
            "ttt" => ttt_move_outcome(self.get_moves_vec_str(), &new_move),
            "uttt" => MoveOutcome::None,
            "c4" => MoveOutcome::None,
            "pc" => MoveOutcome::None,
            _ => MoveOutcome::None,
        }
    }
}
