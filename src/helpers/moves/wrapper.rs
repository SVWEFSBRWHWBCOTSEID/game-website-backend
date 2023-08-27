use crate::prisma::game;
use crate::helpers::moves::uttt::validate_uttt_move;
use super::ttt::{validate_ttt_move};


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
}
