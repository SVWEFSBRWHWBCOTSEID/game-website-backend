use std::collections::HashMap;


const KEY_NAMES: HashMap<&str, &str> = HashMap::from([
    ("ttt", "Tic-Tac-Toe"),
    ("uttt", "Ultimate Tic-Tac-Toe"),
    ("pc", "Pokemon Chess"),
]);

pub fn get_key_name(key: &str) -> String {
    KEY_NAMES.get(key).unwrap().to_string()
}

