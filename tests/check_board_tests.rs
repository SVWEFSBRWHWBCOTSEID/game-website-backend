use game_backend::helpers::moves::ttt::{check_board_status, PlayerSymbol};
use game_backend::models::general::MoveOutcome;


// [ o _ o ]
// [ _ x o ]
// [ x X x ]
#[test]
fn win_on_row_ttt() {
    let board = vec![
        PlayerSymbol::Second, PlayerSymbol::Empty, PlayerSymbol::Second,
        PlayerSymbol::Empty, PlayerSymbol::First, PlayerSymbol::Second,
        PlayerSymbol::First, PlayerSymbol::First, PlayerSymbol::First,
    ];

    assert_eq!(
        check_board_status(7, 7, &board, 3, 3, 3),
        MoveOutcome::FirstWin,
    )
}

// [ x _ _ ]
// [ X o _ ]
// [ x o _ ]
#[test]
fn win_on_column_ttt() {
    let board = vec![
        PlayerSymbol::First, PlayerSymbol::Empty, PlayerSymbol::Empty,
        PlayerSymbol::First, PlayerSymbol::Second, PlayerSymbol::Empty,
        PlayerSymbol::First, PlayerSymbol::Second, PlayerSymbol::Empty,
    ];

    assert_eq!(
        check_board_status(3, 5, &board, 3, 3, 3),
        MoveOutcome::FirstWin,
    )
}

// [ o _ X ]
// [ _ x o ]
// [ x o x ]
#[test]
fn win_on_diag_ttt() {
    let board = vec![
        PlayerSymbol::Second, PlayerSymbol::Empty, PlayerSymbol::First,
        PlayerSymbol::Empty, PlayerSymbol::First, PlayerSymbol::Second,
        PlayerSymbol::First, PlayerSymbol::Second, PlayerSymbol::First,
    ];

    assert_eq!(
        check_board_status(2, 7, &board, 3, 3, 3),
        MoveOutcome::FirstWin,
    )
}

// [ X _ o ]
// [ _ x o ]
// [ x o x ]
#[test]
fn win_on_anti_diag() {
    let board = vec![
        PlayerSymbol::First, PlayerSymbol::Empty, PlayerSymbol::Second,
        PlayerSymbol::Empty, PlayerSymbol::First, PlayerSymbol::Second,
        PlayerSymbol::First, PlayerSymbol::Second, PlayerSymbol::First,
    ];

    assert_eq!(
        check_board_status(0, 7, &board, 3, 3, 3),
        MoveOutcome::FirstWin,
    )
}

// [ x _ _ ]
// [ _ O o ]
// [ o x x ]
#[test]
fn row_doesnt_wrap_ttt() {
    let board = vec![
        PlayerSymbol::First, PlayerSymbol::Empty, PlayerSymbol::Empty,
        PlayerSymbol::Empty, PlayerSymbol::Second, PlayerSymbol::Second,
        PlayerSymbol::Second, PlayerSymbol::First, PlayerSymbol::First,
    ];

    assert_eq!(
        check_board_status(4, 6, &board, 3, 3, 3),
        MoveOutcome::None,
    )
}

// [ o _ _ ]
// [ _ x o ]
// [ X _ x ]
#[test]
fn anti_diag_doesnt_wrap_ttt() {
    let board = vec![
        PlayerSymbol::Second, PlayerSymbol::Empty, PlayerSymbol::Empty,
        PlayerSymbol::Empty, PlayerSymbol::First, PlayerSymbol::Second,
        PlayerSymbol::First, PlayerSymbol::Empty, PlayerSymbol::First,
    ];

    assert_eq!(
        check_board_status(6, 5, &board, 3, 3, 3),
        MoveOutcome::None,
    )
}

// [ _ _ _ _ ]
// [ _ x o _ ]
// [ X o o x ]
// [ _ o x _ ]
#[test]
fn anti_diag_doesnt_wrap_large_board() {
    let board = vec![
        PlayerSymbol::Empty, PlayerSymbol::Empty, PlayerSymbol::Empty, PlayerSymbol::Empty,
        PlayerSymbol::Empty, PlayerSymbol::First, PlayerSymbol::Second, PlayerSymbol::Empty,
        PlayerSymbol::First, PlayerSymbol::Second, PlayerSymbol::Second, PlayerSymbol::First,
        PlayerSymbol::Empty, PlayerSymbol::Second, PlayerSymbol::First, PlayerSymbol::Empty,
    ];

    assert_eq!(
        check_board_status(8, 8, &board, 4, 4, 4),
        MoveOutcome::None
    )
}
