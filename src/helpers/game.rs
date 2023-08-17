use std::cmp::max;
use actix_web::web;
use glicko_2::game::compete;
use glicko_2::Rating;

use crate::models::res::{CreateGameResponse, GameResponse, LobbyResponse};
use crate::models::general::{TimeControl, Player, GameStatus, GameType, Offer, GameKey, EndType, Side, MoveOutcome, GamePerf};
use crate::models::events::{GameState, GameFullEvent, GameEventType, Visibility, Chat};
use crate::prisma::{game, PrismaClient, user, perf};
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
                    provisional: self.first_prov.unwrap(),
                    rating: self.first_rating.unwrap(),
                }),
                None => None,
            },
            second_player: match game.second_user().unwrap() {
                Some(u) => Some(Player {
                    username: u.username.clone(),
                    provisional: self.second_prov.unwrap(),
                    rating: self.second_rating.unwrap(),
                }),
                None => None,
            },
            start_pos: self.start_pos.clone(),
            game_state: GameState {
                ftime: self.get_new_first_time()?,
                stime: self.get_new_second_time()?,
                moves: self.get_moves_vec(),
                status: GameStatus::from_str(&self.status)?,
                end_type: None,
                draw_offer: Offer::None,
                frating_diff: self.get_rating_diffs(GameStatus::from_str(&self.status)?)?.0,
                srating_diff: self.get_rating_diffs(GameStatus::from_str(&self.status)?)?.1,
            },
        })
    }

    // method to construct game response object for fetching a game by id
    pub fn to_game_res(&self) -> Result<GameResponse, WebErr> {
        Ok(GameResponse {
            id: self.id.clone(),
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
                    provisional: self.first_prov.unwrap(),
                    rating: self.first_rating.unwrap(),
                }),
                None => None,
            },
            second: match self.second_user().unwrap() {
                Some(u) => Some(Player {
                    username: u.username.clone(),
                    provisional: self.second_prov.unwrap(),
                    rating: self.second_rating.unwrap(),
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
                provisional: self.first_prov.unwrap(),
                rating: self.first_rating.unwrap(),
            },
            second: Player {
                username: self.second_username.clone().unwrap(),
                provisional: self.second_prov.unwrap(),
                rating: self.second_rating.unwrap(),
            },
            chat: self.chat.clone().unwrap_or(vec![]).iter().map(|x| Ok::<Chat, WebErr>(if x.game_event {
                Chat::ChatAlert {
                    message: x.text.clone(),
                }
            } else {
                Chat::ChatMessage {
                    username: x.username.clone(),
                    text: x.text.clone(),
                    visibility: Visibility::from_str(&x.visibility)?,
                }
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
                end_type: match &self.win_type {
                    Some(wt) => Some(EndType::from_str(wt)?),
                    None => None,
                },
                draw_offer: Offer::from_str(&self.draw_offer)?,
                frating_diff: self.get_rating_diffs(GameStatus::from_str(&self.status)?)?.0,
                srating_diff: self.get_rating_diffs(GameStatus::from_str(&self.status)?)?.1,
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
                    provisional: self.first_prov.unwrap(),
                    rating: self.first_rating.unwrap(),
                },
                None => {
                    let u = self.second_user().or(Err(WebErr::Internal(format!("second_user not fetched"))))?.unwrap();
                    Player {
                        username: u.username.clone(),
                        provisional: self.second_prov.unwrap(),
                        rating: self.second_rating.unwrap(),
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

    // Asserts that the provided user is in the game, and it has started.
    pub fn validate(&self, username: &str) -> Result<game::Data, WebErr> {
        if GameStatus::from_str(&self.status)? != GameStatus::Started ||
            self.first_username.clone().unwrap() != username && self.second_username.clone().unwrap() != username {
            Err(WebErr::Forbidden(format!("could not validate, game not started or not a player")))
        } else {
            Ok(self.clone())
        }
    }

    // Asserts that the provided user is in the game, and it has ended.
    pub fn validate_ended(&self, username: &str) -> Result<game::Data, WebErr> {
        let status = GameStatus::from_str(&self.status)?;
        if status != GameStatus::FirstWon && status != GameStatus::SecondWon && status != GameStatus::Draw ||
            self.first_username.clone().unwrap() != username && self.second_username.clone().unwrap() != username {
            Err(WebErr::Forbidden(format!("could not validate, game not ended or not a player")))
        } else {
            Ok(self.clone())
        }
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

    pub fn get_new_move_status(&self, new_move: &str) -> Result<GameStatus, WebErr> {
        Ok(match self.new_move_outcome(new_move) {
            MoveOutcome::None => GameStatus::Started,
            MoveOutcome::FirstWin => GameStatus::FirstWon,
            MoveOutcome::SecondWin => GameStatus::SecondWon,
            _ => GameStatus::Draw,
        })
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
    
    pub fn get_rating_diffs(&self, new_status: GameStatus) -> Result<(Option<i32>, Option<i32>), WebErr> {
        if !self.rated {
            return Ok((None, None));
        }

        let first_user = self.first_user().or(Err(WebErr::Internal(format!("first user not fetched"))))?.unwrap();
        let second_user = self.first_user().or(Err(WebErr::Internal(format!("second user not fetched"))))?.unwrap();

        let first_tuning = first_user.get_tuning(&self.game_key)?;
        let second_tuning = second_user.get_tuning(&self.game_key)?;

        let mut first_rating = Rating::new(&first_tuning);
        let mut second_rating = Rating::new(&second_tuning);
        let first_old = first_rating.mu;
        let second_old = second_rating.mu;

        match new_status {
            GameStatus::FirstWon => compete(&mut first_rating, &mut second_rating, false),
            GameStatus::SecondWon => compete(&mut second_rating, &mut first_rating, false),
            GameStatus::Draw => compete(&mut first_rating, &mut second_rating, true),
            _ => {},
        }
        Ok((Some((first_rating.mu - first_old) as i32), Some((second_rating.mu - second_old) as i32)))
    }

    pub async fn update_ratings(&self, client: &web::Data<PrismaClient>, new_status: GameStatus) -> Result<(), WebErr> {
        let first_user = self.first_user().or(Err(WebErr::Internal(format!("first user not fetched"))))?.unwrap();
        let second_user = self.second_user().or(Err(WebErr::Internal(format!("second user not fetched"))))?.unwrap();
        let first_tuning = first_user.get_tuning(&self.game_key)?;
        let second_tuning = second_user.get_tuning(&self.game_key)?;
        let mut first_rating = Rating::new(&first_tuning);
        let mut second_rating = Rating::new(&second_tuning);
        let first_old = first_rating.mu;
        let second_old = second_rating.mu;

        match new_status {
            GameStatus::FirstWon => compete(&mut first_rating, &mut second_rating, false),
            GameStatus::SecondWon => compete(&mut second_rating, &mut first_rating, false),
            GameStatus::Draw => compete(&mut first_rating, &mut second_rating, true),
            _ => {},
        }

        let mut first_prog = GamePerf::prog_from_str(&first_user.get_prog(&self.game_key)?)?;
        let mut second_prog = GamePerf::prog_from_str(&second_user.get_prog(&self.game_key)?)?;
        first_prog.push(first_rating.mu - first_old);
        second_prog.push(second_rating.mu - second_old);
        first_prog.remove(0);
        second_prog.remove(0);

        client
            .perf()
            .update(
                perf::username_game_key(self.first_username.clone().unwrap(), self.game_key.clone()),
                vec![
                    perf::rating::set(first_rating.mu),
                    perf::rd::set(first_rating.phi),
                    perf::volatility::set(first_rating.sigma),
                    perf::prog::set(GamePerf::stringify_prog(first_prog)),
                ],
            )
            .exec()
            .await
            .or(Err(WebErr::Internal(format!("error updating perfs for user {}", self.first_username.clone().unwrap()))))?;
        client
            .perf()
            .update(
                perf::username_game_key(self.second_username.clone().unwrap(), self.game_key.clone()),
                vec![
                    perf::rating::set(second_rating.mu),
                    perf::rd::set(second_rating.phi),
                    perf::volatility::set(second_rating.sigma),
                    perf::prog::set(GamePerf::stringify_prog(second_prog)),
                ],
            )
            .exec()
            .await
            .or(Err(WebErr::Internal(format!("error updating perfs for user {}", self.second_username.clone().unwrap()))))?;
        Ok(())
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
