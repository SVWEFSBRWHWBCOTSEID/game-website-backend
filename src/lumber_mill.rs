use std::collections::HashMap;
use actix_web::web::Data;
use parking_lot::Mutex;

use crate::common::WebErr;
use crate::helpers::moves::ttt::{TTTSymbol, process_ttt_move, validate_ttt_move};
use crate::helpers::moves::uttt::{process_uttt_move, validate_uttt_move};
use crate::models::general::MoveOutcome;
use crate::prisma::game;

use self::GameBoard::{TTTBoard, UTTTBoard};


pub struct LumberMill {
    pub boards: HashMap<String, GameBoard>,
}

#[derive(Clone)]
pub enum GameBoard {
    TTTBoard(Vec<TTTSymbol>),
    UTTTBoard(Vec<Vec<TTTSymbol>>, Vec<MoveOutcome>, i32)
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

    // Creates a new board populated from a given prisma game object.
    pub fn create_board_from_game(&mut self, game: &game::Data) -> Result<(), WebErr> {
        let mut board = match game.game_key.as_str() {
            "ttt" => TTTBoard(empty_ttt_board()),
            "uttt" => UTTTBoard(
                vec![empty_ttt_board(); 9],
                vec![MoveOutcome::None; 9],
                4
            ),
            _ => return Err(WebErr::BadReq(format!("game does not exist or is not supported")))
        };

        // Populate board with moves
        let mut is_first = true;
        for m in game.get_moves_vec_str() {
            process_move(&mut board, m, is_first);
            is_first = !is_first;
        }

        self.boards.insert(game.id.clone(), board);
        Ok(())
    }

    // Gets whether a move is valid given a game board.
    pub fn validate_move(&mut self, game: &game::Data, new_move: &str) -> Result<bool, WebErr> {
        if !self.boards.contains_key(game.id.as_str()) {
            self.create_board_from_game(&game)?;
        }

        let board = self.boards.get(game.id.as_str()).unwrap();
        Ok(match board {
            // TODO: use board checking instead of moves vec?
            TTTBoard(_) =>
                validate_ttt_move(new_move, game.get_moves_vec_str()),
            UTTTBoard(_, game_states, active_board) =>
                validate_uttt_move(new_move, game.get_moves_vec_str(), game_states, *active_board)
        })
    }

    // Updates the given game's board with the provided move, checking the new board for victories
    // and returning the resultant `MoveOutcome`.
    pub fn update_and_check(&mut self, game: &game::Data, new_move: &str, is_first: bool) -> Result<MoveOutcome, WebErr> {
        if !self.boards.contains_key(game.id.as_str()) {
            self.create_board_from_game(&game)?;
        }

        Ok(process_move(
            self.boards.get_mut(game.id.as_str()).unwrap(),
            new_move,
            is_first
        ))
    }
}

// Processes a move string, mutating the given game board with the new state
// and returning the resultant `MoveOutcome`.
fn process_move(board: &mut GameBoard, new_move: &str, is_first: bool) -> MoveOutcome {
    match board {
        TTTBoard(board) =>
            process_ttt_move(new_move, board, is_first),
        UTTTBoard(board, board_states, active_board) =>
            process_uttt_move(new_move, board, board_states, active_board, is_first),
        // "c4" => MoveOutcome::None,
        // "pc" => MoveOutcome::None,
    }
}

fn empty_ttt_board() -> Vec<TTTSymbol> {
    vec![TTTSymbol::Empty; 9]
}
