use crate::models::general::MoveOutcome;


pub fn validate_ttt_move(moves: Vec<&str>, new_move: &str) -> bool {
    !moves.contains(&new_move)
}

#[derive(PartialEq, Clone)]
pub enum TTTSymbol {
    X, O, Empty
}

pub fn check_board_status(m: usize, move_num: usize, board: &Vec<TTTSymbol>, rows: usize, columns: usize, needed: usize) -> MoveOutcome {
    // TODO: abstract this with the passed-in `is_first` in `wrapper.rs`?
    let x_move = move_num % 2 == 0;
    if move_num == board.len() {
        return MoveOutcome::Draw
    }

    // Rows
    let row_start = m - (m % columns);
    for i in (m - needed).max(row_start)..(m + needed).min(row_start + columns) {
        let mut cond = board[i] != TTTSymbol::Empty;
        for j in 1..needed {
            cond = cond && board[i] == board[i + j];
        }

        if cond {
            return if x_move {
                MoveOutcome::FirstWin
            } else {
                MoveOutcome::SecondWin
            };
        }
    }

    // Columns
    let col_start = m % columns;
    for i in ((m - (needed * columns)).max(col_start)..(m + (needed * columns) + 1).min(board.len())).step_by(columns) {
        let mut cond = board[i] != TTTSymbol::Empty;
        for j in 1..needed {
            cond = cond && board[i] == board[i + (j * columns)];
        }

        if cond {
            return if x_move {
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
        for i in diag_start..(m + (needed * (columns + 1)) + 1).min(board.len()) {
            let mut cond = board[i] != TTTSymbol::Empty;
            for j in 1..needed {
                cond = cond && board[i] == board[i + (j * (columns + 1))];
            }

            if cond {
                return if x_move {
                    MoveOutcome::FirstWin
                } else {
                    MoveOutcome::SecondWin
                };
            }
        }
    }

    // Anti-diagonal
    // TODO: condition for checking antidiag?
    // if (Math.abs(rowNum + colStart) >= needed)
    let anti_diag_start = if row_num + col_start >= columns {
        ((row_num + col_start) - (columns - 1)) * columns + (columns - 1)
    } else {
        row_num + col_start
    };

    for i in (anti_diag_start..(m + (needed * (columns - 1)) + 1).min(board.len())).step_by(columns - 1) {
        let mut cond = board[i] != TTTSymbol::Empty;
        for j in 1..needed {
            cond = cond && board[i] == board[i + (j * (columns - 1))];
        }

        if cond {
            return if x_move {
                MoveOutcome::FirstWin
            } else {
                MoveOutcome::SecondWin
            };
        }
    }

    return MoveOutcome::None;
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
