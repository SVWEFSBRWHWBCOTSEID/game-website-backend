use std::collections::HashMap;
use actix_web::web::Data;
use parking_lot::Mutex;

use crate::helpers::moves::ttt::TTTSymbol;


pub struct LumberMill {
    pub boards: HashMap<String, Vec<TTTSymbol>>,
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

    // Creates a new ttt board given a game id
    pub fn create_new_ttt_board(&mut self, game_id: String) {
        self.boards.insert(game_id, vec![TTTSymbol::Empty; 9]);
    }
}
