use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct Move {
    id: Option<i64>,
    name: Option<String>,
}

