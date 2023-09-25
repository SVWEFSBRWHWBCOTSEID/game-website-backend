use log::debug;
use crate::models::general::MoveOutcome;


#[derive(PartialEq, Clone, Copy)]
pub enum PlayerSymbol {
    First, Second, Empty
}

// Validates a ttt move. A move is invalid if:
// 1. It is not in the correct format ("a1")
// 2. It has already been played
pub fn validate_ttt_move(new_move: &str, moves: Vec<&str>) -> bool {
    !moves.contains(&new_move)
        && new_move.chars().nth(0).is_some_and(|c| matches!(c, 'a'..='c'))
        && new_move.chars().nth(1).is_some_and(|c| c.is_digit(10))
}

pub fn process_ttt_move(new_move: &str, board: &mut Vec<PlayerSymbol>, is_first: bool) -> MoveOutcome {
    let m = col_to_index(new_move.chars().nth(0).unwrap())
        + row_to_index(new_move.chars().nth(1).unwrap()) * 3;

    board[m] = if is_first {
        PlayerSymbol::First
    } else {
        PlayerSymbol::Second
    };

    // TODO: better move num calc?
    let move_num = board.iter().filter(|m| **m != PlayerSymbol::Empty).count();
    check_board_status(m, move_num, board, 3, 3, 3)
}

pub fn check_board_status(m: usize, move_num: usize, board: &Vec<PlayerSymbol>, rows: usize, columns: usize, needed: usize) -> MoveOutcome {
    if move_num == board.len() {
        return MoveOutcome::Draw
    }

    // Rows
    let row_start = m - (m % columns);

    // TODO: less hacky "signed subtraction" between usizes?
    let rstart = (m as i32 - needed as i32).max(row_start as i32) as usize;
    for i in rstart..=m {
        let mut cond = board[i] != PlayerSymbol::Empty;
        for j in 1..needed {
            let index = i + j;
            cond = cond && index < board.len() && board[i] == board[index];
        }

        if cond {
            debug!("won on row");
            return if board[i] == PlayerSymbol::First {
                MoveOutcome::FirstWin
            } else {
                MoveOutcome::SecondWin
            };
        }
    }

    // Columns
    let col_start = m % columns;

    // TODO: less hacky "signed subtraction" between usizes?
    let cstart = (m as i32 - (needed * columns) as i32).max(col_start as i32) as usize;
    for i in (cstart..=m).step_by(columns) {
        let mut cond = board[i] != PlayerSymbol::Empty;
        for j in 1..needed {
            let index = i + (j * columns);
            cond = cond && index < board.len() && board[i] == board[index];
        }

        if cond {
            debug!("won on column");
            return if board[i] == PlayerSymbol::First {
                MoveOutcome::FirstWin
            } else {
                MoveOutcome::SecondWin
            };
        }
    }

    // Diagonal
    let row_num = row_start / columns;
    let diag_start = if row_num > col_start {
        (row_num - col_start) * columns
    } else {
        col_start - row_num
    };

    if rows.min(columns) - row_num.abs_diff(col_start) >= needed {
        for i in (diag_start..=m).step_by(columns + 1) {
            let mut cond = board[i] != PlayerSymbol::Empty;
            for j in 1..needed {
                let index = i + (j * (columns + 1));
                cond = cond && index < board.len() && board[i] == board[index];
            }

            if cond {
                debug!("won on diagonal");
                return if board[i] == PlayerSymbol::First {
                    MoveOutcome::FirstWin
                } else {
                    MoveOutcome::SecondWin
                };
            }
        }
    }

    // Anti-diagonal
    let anti_diag_start = if row_num + col_start >= columns {
        ((row_num + col_start) - (columns - 1)) * columns + (columns - 1)
    } else {
        row_num + col_start
    };

    // TODO: row check?
    if anti_diag_start >= needed - 1 {
        for i in (anti_diag_start..=m).step_by(columns - 1) {
            let mut cond = board[i] != PlayerSymbol::Empty;
            for j in 1..needed {
                let index = i + (j * (columns - 1));
                if index % columns == 0 && j != needed - 1 {
                    cond = false;
                    break
                }

                cond = cond && index < board.len() && board[i] == board[index];
            }

            if cond {
                debug!("won on anti-diagonal");
                return if board[i] == PlayerSymbol::First {
                    MoveOutcome::FirstWin
                } else {
                    MoveOutcome::SecondWin
                };
            }
        }
    }

    MoveOutcome::None
}

pub fn row_to_index(row: char) -> usize {
    row.to_digit(10).unwrap() as usize - 1
}

pub fn col_to_index(col: char) -> usize {
    col as usize - 97
}
