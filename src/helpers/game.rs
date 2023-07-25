use std::cmp::max;
use actix_web::web;

use crate::models::res::{CreateGameResponse, GameResponse, LobbyResponse};
use crate::models::general::{TimeControl, Player, GameStatus, GameType, Offer, GameKey, WinType, Side};
use crate::models::events::{GameState, GameFullEvent, GameEventType, Visibility, ChatMessage};
use crate::prisma::{game, PrismaClient, user};
use crate::common::WebErr;
use super::general::time_millis;


impl game::Data {
    // method to construct response from prisma game struct
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
                ftime: self.get_new_first_time()?,
                stime: self.get_new_second_time()?,
                moves: self.get_moves_vec(),
                status: GameStatus::from_str(&self.status)?,
                win_type: None,
                draw_offer: Offer::None,
            },
        })
    }

    // method to construct game response object for fetching a game by id
    pub fn to_game_res(&self) -> Result<GameResponse, WebErr> {
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
            first: match self.first_user().unwrap() {
                Some(u) => Some(Player {
                    username: u.username.clone(),
                    provisional: u.get_provisional(&self.game_key)?,
                    rating: u.get_rating(&self.game_key)?,
                }),
                None => None,
            },
            second: match self.second_user().unwrap() {
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
            chat: self.chat.clone().unwrap_or(vec![]).iter().map(|x| Ok::<ChatMessage, WebErr>(ChatMessage {
                username: x.username.clone(),
                text: x.text.clone(),
                visibility: Visibility::from_str(&x.visibility)?,
            })).flatten().collect(),
            state: GameState {
                ftime: self.get_new_first_time()?,
                stime: self.get_new_second_time()?,
                moves: if self.moves.len() > 0 {
                    self.moves.split(" ").map(|s| s.to_string()).collect()
                } else {
                    vec![]
                },
                status: GameStatus::from_str(&self.status)?,
                win_type: match &self.win_type {
                    Some(wt) => Some(WinType::from_str(wt)?),
                    None => None,
                },
                draw_offer: Offer::from_str(&self.draw_offer)?,
            },
        })
    }

    pub fn to_lobby_res(&self, random: bool) -> Result<LobbyResponse, WebErr> {
        Ok(LobbyResponse {
            id: self.id.clone(),
            rated: self.rated,
            rating_min: self.rating_min,
            rating_max: self.rating_max,
            side: if random {
                Side::Random
            } else if self.first_username.is_some() {
                Side::First
            } else {
                Side::Second
            },
            user: match self.first_user().or(Err(WebErr::Internal(format!("first_user not fetched"))))? {
                Some(u) => Player {
                    username: u.username.clone(),
                    provisional: u.get_provisional(&self.game_key)?,
                    rating: u.get_rating(&self.game_key)?,
                },
                None => {
                    let u = self.second_user().or(Err(WebErr::Internal(format!("second_user not fetched"))))?.unwrap();
                    Player {
                        username: u.username.clone(),
                        provisional: u.get_provisional(&self.game_key)?,
                        rating: u.get_rating(&self.game_key)?,
                    }
                },
            },
            game: GameType {
                key: self.game_key.clone(),
                name: GameKey::get_game_name(&self.game_key)?,
            },
            time_control: TimeControl {
                initial: self.clock_initial,
                increment: self.clock_increment,
            },
        })
    }

    // helpers to get updated first and second times
    pub fn get_new_first_time(&self) -> Result<Option<i32>, WebErr> {
        if GameStatus::from_str(&self.status)? != GameStatus::Started {
            return Ok(self.first_time);
        }
        Ok(match self.first_time {
            Some(t) => if self.num_moves() >= 2 && self.num_moves() % 2 == 0 {
                Some(max(0, t - (time_millis() - self.last_move_time) as i32 + self.clock_increment.unwrap()))
            } else {
                Some(t)
            },
            None => None,
        })
    }

    pub fn get_new_second_time(&self) -> Result<Option<i32>, WebErr> {
        if GameStatus::from_str(&self.status)? != GameStatus::Started {
            return Ok(self.second_time);
        }
        match self.second_time {
            Some(t) => if self.num_moves() >= 2 && self.num_moves() % 2 == 1 {
                Ok(Some(max(0, t - (time_millis() - self.last_move_time) as i32 + self.clock_increment.unwrap())))
            } else {
                Ok(Some(t))
            },
            None => Ok(None),
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

    pub fn get_draw_game_status(&self, value: &bool, username: &str) -> Result<GameStatus, WebErr> {
        Ok(match (
            self.first_username.clone().unwrap() == username,
            value,
            Offer::from_str(&self.draw_offer)?,
        ) {
            (true, true, Offer::Second) => GameStatus::Draw,
            (false, true, Offer::First) => GameStatus::Draw,
            (true, false, Offer::Second) => GameStatus::Started,
            (false, false, Offer::First) => GameStatus::Started,
            _ => GameStatus::from_str(&self.status)?,
        })
    }

    pub fn get_resign_game_status(&self, username: &str) -> GameStatus {
        if self.first_username.clone().unwrap() == username {
            GameStatus::SecondWon
        } else {
            GameStatus::FirstWon
        }
    }

    pub fn get_timeout_game_status(&self, username: &str) -> Result<GameStatus, WebErr> {
        Ok(if self.first_username.clone().unwrap() == username && self.get_new_first_time()?.unwrap() <= 0 {
            GameStatus::SecondWon
        } else if self.second_username.clone().unwrap() == username && self.get_new_second_time()?.unwrap() <= 0 {
            GameStatus::FirstWon
        } else {
            GameStatus::from_str(&self.status)?
        })
    }

    pub fn get_new_draw_offer(&self, value: &bool, username: &str) -> Result<Offer, WebErr> {
        Ok(match (
            self.first_username.clone().unwrap() == username,
            value,
            Offer::from_str(&self.draw_offer)?,
        ) {
            (true, true, Offer::None) => Offer::First,
            (false, true, Offer::None) => Offer::Second,
            (true, true, Offer::Second) => Offer::Agreed,
            (false, true, Offer::First) => Offer::Agreed,
            (true, false, Offer::Second) => Offer::None,
            (false, false, Offer::First) => Offer::None,
            _ => Offer::from_str(&self.draw_offer)?,
        })
    }

    pub fn get_new_rematch_offer(&self, value: &bool, username: &str) -> Result<Offer, WebErr> {
        Ok(match (
            self.first_username.clone().unwrap() == username,
            value,
            Offer::from_str(&self.rematch_offer)?,
        ) {
            (true, true, Offer::None) => Offer::First,
            (false, true, Offer::None) => Offer::Second,
            (true, true, Offer::Second) => Offer::Agreed,
            (false, true, Offer::First) => Offer::Agreed,
            (true, false, Offer::Second) => Offer::None,
            (false, false, Offer::First) => Offer::None,
            _ => Offer::from_str(&self.rematch_offer)?,
        })
    }
}

pub trait LobbyVec {
    fn to_lobby_vec(&self) -> Result<Vec<LobbyResponse>, WebErr>;
}

impl LobbyVec for Vec<game::Data> {
    // convert vec of games to vec of LobbyResponse structs
    fn to_lobby_vec(&self) -> Result<Vec<LobbyResponse>, WebErr> {
        Ok(self.iter().map(
            |g| Ok::<LobbyResponse, WebErr>(g.to_lobby_res(g.random_side)?)
        ).flatten().collect())
    }
}
