pub mod app_config;
pub mod models;
pub mod handlers;
pub mod prisma;

use phf::phf_map;


static KEY_NAMES: phf::Map<&'static str, &'static str> = phf_map! {
    "ttt" => "Tic-Tac-Toe",
    "uttt" => "Ultimate Tic-Tac-Toe",
    "pc" => "Pokemon Chess",
};

pub fn get_key_name(key: &str) -> String {
    KEY_NAMES.get(key).unwrap().to_string()
}
