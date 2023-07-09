use std::cmp::max;
use actix_web::web;

use crate::{models::{res::{CreateGameResponse, GameResponse}, general::{TimeControl, Player, GameStatus, GameType, DrawOffer}, events::GameState}, prisma::{game, PrismaClient}};
use super::general::{get_key_name, time_millis};


impl game::Data {
    // method to construct reponse from prisma game struct
    pub async fn to_create_game_res(&self, client: &web::Data<PrismaClient>) -> CreateGameResponse {
        // get game from table to get user relations
        let game = client
            .game()
            .find_unique(game::id::equals(self.id.clone()))
            .with(game::first_user::fetch())
            .with(game::second_user::fetch())
            .exec()
            .await
            .unwrap()
            .unwrap();

        CreateGameResponse {
            id: self.id.clone(),
            created_at: self.created_at.to_string(),
            rated: self.rated,
            game: GameType {
                key: self.game_key.clone(),
                name: get_key_name(&self.game_key).unwrap(),
            },
            time_control: TimeControl {
                initial: self.clock_initial,
                increment: self.clock_increment,
            },
            first_player: match game.first_user().unwrap() {
                Some(u) => Some(Player {
                    username: u.username.clone(),
                    provisional: u.get_provisional(&self.game_key).unwrap(),
                    rating: u.get_rating(&self.game_key).unwrap(),
                }),
                None => None,
            },
            second_player: match game.second_user().unwrap() {
                Some(u) => Some(Player {
                    username: u.username.clone(),
                    provisional: u.get_provisional(&self.game_key).unwrap(),
                    rating: u.get_rating(&self.game_key).unwrap(),
                }),
                None => None,
            },
            start_pos: self.start_pos.clone(),
            game_state: GameState {
                ftime: self.get_new_first_time(),
                stime: self.get_new_second_time(),
                moves: self.get_moves_vec(),
                status: GameStatus::from_str(&self.status),
                win_type: None,
                draw_offer: DrawOffer::None,
            },
        }
    }

    // method to construct game response object for fetching a game by id
    pub async fn to_game_res(&self, client: &web::Data<PrismaClient>) -> GameResponse {
        // get game from table to get user relations
        let game = client
            .game()
            .find_unique(game::id::equals(self.id.clone()))
            .with(game::first_user::fetch())
            .with(game::second_user::fetch())
            .exec()
            .await
            .unwrap()
            .unwrap();

        GameResponse {
            rated: self.rated,
            game: GameType {
                key: self.game_key.clone(),
                name: get_key_name(&self.game_key).unwrap(),
            },
            time_control: TimeControl {
                initial: self.clock_initial,
                increment: self.clock_increment,
            },
            created_at: self.created_at.to_string(),
            first: match game.first_user().unwrap() {
                Some(u) => Some(Player {
                    username: u.username.clone(),
                    provisional: u.get_provisional(&self.game_key).unwrap(),
                    rating: u.get_rating(&self.game_key).unwrap(),
                }),
                None => None,
            },
            second: match game.second_user().unwrap() {
                Some(u) => Some(Player {
                    username: u.username.clone(),
                    provisional: u.get_provisional(&self.game_key).unwrap(),
                    rating: u.get_rating(&self.game_key).unwrap(),
                }),
                None => None,
            },
        }
    }

    // helpers to get updated first and second times
    pub fn get_new_first_time(&self) -> Option<i32> {
        match self.first_time {
            Some(t) => if self.num_moves() >= 2 && self.num_moves() % 2 == 0 {
                Some(max(0, t - (time_millis() - self.last_move_time) as i32))
            } else {
                Some(t)
            },
            None => None,
        }
    }

    pub fn get_new_second_time(&self) -> Option<i32> {
        match self.second_time {
            Some(t) => if self.num_moves() >= 2 && self.num_moves() % 2 == 1 {
                Some(max(0, t - (time_millis() - self.last_move_time) as i32))
            } else {
                Some(t)
            },
            None => None,
        }
    }

    // helper to get number of moves
    pub fn num_moves(&self) -> usize {
        if self.moves.len() == 0 {
            return 0;
        }
        self.moves.split(" ").collect::<Vec<&str>>().len()
    }

    // helper to convert moves string to vec
    pub fn get_moves_vec(&self) -> Vec<String> {
        if self.moves.len() > 0 {
            self.moves.split(" ").map(|s| s.to_string()).collect()
        } else {
            vec![]
        }
    }

    // same as get_moves_vec but with &str rather than String
    pub fn get_moves_vec_str(&self) -> Vec<&str> {
        if self.moves.len() > 0 {
            self.moves.split(" ").collect()
        } else {
            vec![]
        }
    }

    pub fn get_draw_game_status(&self, value: &bool, username: &str) -> GameStatus {
        match (
            self.first_username.clone().unwrap() == username,
            value,
            self.draw_offer,
        ) {
            (true, true, Some(false)) => GameStatus::Draw,
            (false, true, Some(true)) => GameStatus::Draw,
            (true, false, Some(false)) => GameStatus::Started,
            (false, false, Some(true)) => GameStatus::Started,
            _ => GameStatus::from_str(&self.status),
        }
    }

    pub fn get_resign_game_status(&self, username: &str) -> GameStatus {
        if self.first_username.clone().unwrap() == username {
            GameStatus::SecondWon
        } else {
            GameStatus::FirstWon
        }
    }

    pub fn get_timeout_game_status(&self, username: &str) -> GameStatus {
        if self.first_username.clone().unwrap() == username && self.get_new_first_time().unwrap() <= 0 {
            GameStatus::SecondWon
        } else if self.second_username.clone().unwrap() == username && self.get_new_second_time().unwrap() <= 0 {
            GameStatus::FirstWon
        } else {
            GameStatus::from_str(&self.status)
        }
    }

    pub fn get_new_draw_offer(&self, value: &bool, username: &str) -> DrawOffer {
        match (
            self.first_username.clone().unwrap() == username,
            value,
            self.draw_offer,
        ) {
            (true, true, None) => DrawOffer::First,
            (false, true, None) => DrawOffer::Second,
            (true, true, Some(false)) => DrawOffer::None,
            (false, true, Some(true)) => DrawOffer::None,
            (true, false, Some(false)) => DrawOffer::None,
            (false, false, Some(true)) => DrawOffer::None,
            _ => DrawOffer::from_bool(&self.draw_offer),
        }
    }
}
