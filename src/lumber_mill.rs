use std::collections::HashMap;
use actix_web::web::Data;
use parking_lot::Mutex;

use crate::common::WebErr;
use crate::helpers::moves::ttt::{check_ttt_board_status, col_to_index, row_to_index, TTTSymbol};
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
    match board{
        TTTBoard(board) => {
            let m = col_to_index(new_move.chars().nth(0).unwrap())
                + row_to_index(new_move.chars().nth(1).unwrap()) * 3;

            board[m] = if is_first {
                TTTSymbol::X
            } else {
                TTTSymbol::O
            };

            // TODO: better move num calc?
            let move_num = board.iter().filter(|m| **m != TTTSymbol::Empty).count();
            check_ttt_board_status(m, move_num, board, 3, 3, 3)
        },
        UTTTBoard(board, board_states, active_board) => {
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

fn empty_ttt_board() -> Vec<TTTSymbol> {
    vec![TTTSymbol::Empty; 9]
}
