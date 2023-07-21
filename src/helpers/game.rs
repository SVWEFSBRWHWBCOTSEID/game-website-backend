use std::cmp::max;
use actix_web::web;
use async_trait::async_trait;
use futures::future::join_all;

use crate::{models::{res::{CreateGameResponse, GameResponse}, general::{TimeControl, Player, GameStatus, GameType, DrawOffer, GameKey, WinType}, events::{GameState, GameFullEvent, GameEventType, Visibility, ChatMessage}}, prisma::{game, PrismaClient, user}, common::WebErr};
use super::general::time_millis;


impl game::Data {
    // method to construct reponse from prisma game struct
    pub async fn to_create_game_res(&self, client: &web::Data<PrismaClient>) -> Result<CreateGameResponse, WebErr> {
        // get game from table to get user relations
        let game = client
            .game()
            .find_unique(game::id::equals(self.id.clone()))
            .with(game::first_user::fetch().with(user::perfs::fetch(vec![])))
            .with(game::second_user::fetch().with(user::perfs::fetch(vec![])))
            .exec()
            .await
            .or(Err(WebErr::Internal(format!("could not find game with id {}", self.id))))?
            .unwrap();

        Ok(CreateGameResponse {
            id: self.id.clone(),
            created_at: self.created_at.to_string(),
            rated: self.rated,
            game: GameType {
                key: self.game_key.clone(),
                name: GameKey::get_game_name(&self.game_key)?,
            },
            time_control: TimeControl {
                initial: self.clock_initial,
                increment: self.clock_increment,
            },
            first_player: match game.first_user().unwrap() {
                Some(u) => Some(Player {
                    username: u.username.clone(),
                    provisional: u.get_provisional(&self.game_key)?,
                    rating: u.get_rating(&self.game_key)?,
                }),
                None => None,
            },
            second_player: match game.second_user().unwrap() {
                Some(u) => Some(Player {
                    username: u.username.clone(),
                    provisional: u.get_provisional(&self.game_key)?,
                    rating: u.get_rating(&self.game_key)?,
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
        })
    }

    // method to construct game response object for fetching a game by id
    pub async fn to_game_res(&self, client: &web::Data<PrismaClient>) -> Result<GameResponse, WebErr> {
        // get game from table to get user relations
        let game = client
            .game()
            .find_unique(game::id::equals(self.id.clone()))
            .with(game::first_user::fetch())
            .with(game::second_user::fetch())
            .exec()
            .await
            .or(Err(WebErr::Internal(format!("could not find game with id {}", self.id))))?
            .unwrap();

        Ok(GameResponse {
            rated: self.rated,
            game: GameType {
                key: self.game_key.clone(),
                name: GameKey::get_game_name(&self.game_key)?,
            },
            time_control: TimeControl {
                initial: self.clock_initial,
                increment: self.clock_increment,
            },
            created_at: self.created_at.to_string(),
            first: match game.first_user().unwrap() {
                Some(u) => Some(Player {
                    username: u.username.clone(),
                    provisional: u.get_provisional(&self.game_key)?,
                    rating: u.get_rating(&self.game_key)?,
                }),
                None => None,
            },
            second: match game.second_user().unwrap() {
                Some(u) => Some(Player {
                    username: u.username.clone(),
                    provisional: u.get_provisional(&self.game_key)?,
                    rating: u.get_rating(&self.game_key)?,
                }),
                None => None,
            },
        })
    }

    pub fn to_game_full_event(&self) -> Result<GameFullEvent, WebErr> {
        Ok(GameFullEvent {
            r#type: GameEventType::GameFull,
            rated: self.rated,
            game: GameType {
                key: self.game_key.clone(),
                name: GameKey::get_game_name(&self.game_key)?,
            },
            time_control: TimeControl {
                initial: self.clock_initial,
                increment: self.clock_increment,
            },
            created_at: self.created_at.to_string(),
            first: Player {
                username: self.first_username.clone().unwrap(),
                provisional: self.first_user().unwrap().unwrap().get_provisional(&self.game_key).unwrap(),
                rating: self.first_user().unwrap().unwrap().get_rating(&self.game_key).unwrap(),
            },
            second: Player {
                username: self.second_username.clone().unwrap(),
                provisional: self.second_user().unwrap().unwrap().get_provisional(&self.game_key).unwrap(),
                rating: self.second_user().unwrap().unwrap().get_rating(&self.game_key).unwrap(),
            },
            chat: self.chat.clone().unwrap_or(vec![]).iter().map(|x| ChatMessage {
                username: x.username.clone(),
                text: x.text.clone(),
                visibility: Visibility::from_str(&x.visibility),
            }).collect(),
            state: GameState {
                ftime: self.get_new_first_time(),
                stime: self.get_new_second_time(),
                moves: if self.moves.len() > 0 {
                    self.moves.split(" ").map(|s| s.to_string()).collect()
                } else {
                    vec![]
                },
                status: GameStatus::from_str(&self.status),
                win_type: match &self.win_type {
                    Some(wt) => Some(WinType::from_str(wt)),
                    None => None,
                },
                draw_offer: DrawOffer::from_bool(&self.draw_offer),
            },
        })
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

#[async_trait]
pub trait GameVec {
    async fn to_game_res_vec(&self, client: &web::Data<PrismaClient>) -> Result<Vec<GameResponse>, WebErr>;
}

#[async_trait]
impl GameVec for Vec<game::Data> {
    // convert vec of games to vec of GameResponse structs
    async fn to_game_res_vec(&self, client: &web::Data<PrismaClient>) -> Result<Vec<GameResponse>, WebErr> {
        Ok(join_all(self.iter().map(
            |g| async {
                g.to_game_res(&client).await.expect("Err in to_game_res_vec")
            }
        )).await)
    }
}
