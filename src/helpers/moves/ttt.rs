use log::debug;
use crate::models::general::MoveOutcome;


pub fn validate_ttt_move(moves: Vec<&str>, new_move: &str) -> bool {
    !moves.contains(&new_move)
        && new_move.chars().nth(0).is_some_and(|c| matches!(c, 'a'..='c'))
        && new_move.chars().nth(1).is_some_and(|c| c.is_digit(10))
}

#[derive(PartialEq, Clone, Copy)]
pub enum TTTSymbol {
    X, O, Empty
}

pub fn check_ttt_board_status(m: usize, move_num: usize, board: &Vec<TTTSymbol>, rows: usize, columns: usize, needed: usize) -> MoveOutcome {
    if move_num == board.len() {
        return MoveOutcome::Draw
    }

    // Rows
    let row_start = m - (m % columns);

    // TODO: less hacky "signed subtraction" between usizes?
    let rstart = (m as i32 - needed as i32).max(row_start as i32) as usize;
    let rend = (m + needed).min(row_start + columns);
    for i in rstart..rend {
        let mut cond = board[i] != TTTSymbol::Empty;
        for j in 1..needed {
            let index = i + j;
            cond = cond && index < board.len() && board[i] == board[index];
        }

        if cond {
            debug!("won on row");
            return if board[i] == TTTSymbol::X {
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
    let cend = (m + (needed * columns) + 1).min(board.len());
    for i in (cstart..cend).step_by(columns) {
        let mut cond = board[i] != TTTSymbol::Empty;
        for j in 1..needed {
            let index = i + (j * columns);
            cond = cond && index < board.len() && board[i] == board[index];
        }

        if cond {
            debug!("won on column");
            return if board[i] == TTTSymbol::X {
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
        for i in (diag_start..(m + (needed * (columns + 1)) + 1).min(board.len())).step_by(columns + 1) {
            let mut cond = board[i] != TTTSymbol::Empty;
            for j in 1..needed {
                let index = i + (j * (columns + 1));
                cond = cond && index < board.len() && board[i] == board[index];
            }

            if cond {
                debug!("won on diagonal");
                return if board[i] == TTTSymbol::X {
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
        for i in (anti_diag_start..(m + (needed * (columns - 1)) + 1).min(board.len())).step_by(columns - 1) {
            let mut cond = board[i] != TTTSymbol::Empty;
            for j in 1..needed {
                let index = i + (j * (columns - 1));
                cond = cond && index < board.len() && board[i] == board[index];
            }

            if cond {
                debug!("won on anti-diagonal");
                return if board[i] == TTTSymbol::X {
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
    match col {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        _ => panic!("dies")
    }
}
