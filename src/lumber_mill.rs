use std::collections::HashMap;
use actix_web::web::Data;
use parking_lot::Mutex;

use crate::helpers::moves::ttt::TTTSymbol;
use crate::models::general::MoveOutcome;


pub struct LumberMill {
    pub boards: HashMap<String, GameBoard>,
}

pub enum GameBoard {
    TTTBoard(Vec<TTTSymbol>),
    UTTTBoard(Vec<Vec<TTTSymbol>>, i32, Vec<MoveOutcome>)
}

impl LumberMill {
    pub fn create() -> Data<Mutex<Self>> {
        let mill = Data::new(Mutex::new(LumberMill::new()));
        mill
    }

    fn new() -> Self {
        LumberMill {
            boards: HashMap::new()
        }
    }

    // Creates a new ttt board given a game id.
    pub fn create_new_ttt_board(&mut self, game_id: String) {
        self.boards.insert(game_id, GameBoard::TTTBoard(empty_ttt_board()));
    }

    // Creates a new uttt board and game state given a game id.
    pub fn create_new_uttt_board(&mut self, game_id: String) {
        self.boards.insert(game_id, GameBoard::UTTTBoard(
            vec![empty_ttt_board(); 9],
            4,
            vec![MoveOutcome::None; 9]
        ));
    }
}

fn empty_ttt_board() -> Vec<TTTSymbol> {
    vec![TTTSymbol::Empty; 9]
}
