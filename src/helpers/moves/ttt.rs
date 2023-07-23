use crate::models::general::MoveOutcome;


pub fn validate_ttt_move(moves: Vec<&str>, new_move: &str) -> bool {
    !moves.contains(&new_move)
}

pub fn ttt_move_outcome(moves: Vec<&str>, new_move: &str) -> MoveOutcome {
    let mut total_moves = moves;
    total_moves.push(new_move);
    
    let mut x = [false; 9];
    let mut o = [false; 9];
    for (i, &m) in total_moves.iter().enumerate() {
        match m {
            "a1" => if i % 2 == 0 {x[0] = true} else {o[0] = true},
            "a2" => if i % 2 == 0 {x[1] = true} else {o[1] = true},
            "a3" => if i % 2 == 0 {x[2] = true} else {o[2] = true},
            "b1" => if i % 2 == 0 {x[3] = true} else {o[3] = true},
            "b2" => if i % 2 == 0 {x[4] = true} else {o[4] = true},
            "b3" => if i % 2 == 0 {x[5] = true} else {o[5] = true},
            "c1" => if i % 2 == 0 {x[6] = true} else {o[6] = true},
            "c2" => if i % 2 == 0 {x[7] = true} else {o[7] = true},
            "c3" => if i % 2 == 0 {x[8] = true} else {o[8] = true},
            _ => {},
        }
    }
    if
        x[0] && x[1] && x[2] ||
        x[3] && x[4] && x[5] ||
        x[6] && x[7] && x[8] ||
        x[0] && x[3] && x[6] ||
        x[1] && x[4] && x[7] ||
        x[2] && x[5] && x[8] ||
        x[0] && x[4] && x[8] ||
        x[2] && x[4] && x[6]
    {
        MoveOutcome::FirstWin
    } else if
        o[0] && o[1] && o[2] ||
        o[3] && o[4] && o[5] ||
        o[6] && o[7] && o[8] ||
        o[0] && o[3] && o[6] ||
        o[1] && o[4] && o[7] ||
        o[2] && o[5] && o[8] ||
        o[0] && o[4] && o[8] ||
        o[2] && o[4] && o[6]
    {
        MoveOutcome::SecondWin
    } else if total_moves.len() == 9 {
        MoveOutcome::Draw
    } else {
        MoveOutcome::None
    }
}
