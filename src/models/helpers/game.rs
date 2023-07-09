use std::cmp::max;

use actix_web::web;
use nanoid::nanoid;

use crate::common::{get_key_name, time_millis};
use crate::models::general::{TimeControl, GameState, GameStatus, GameType, MatchPlayer, Player, DrawOffer};
use crate::models::req::CreateGameReq;
use crate::models::res::{CreateGameResponse, GameResponse};
use crate::prisma::PrismaClient;
use crate::prisma::{game, user};


impl CreateGameReq {
    // method to validate this game request
    pub async fn validate(
        &self, 
        client: &web::Data<PrismaClient>,
        player: &MatchPlayer,
    ) -> bool {
        let user = client
            .user()
            .find_unique(user::username::equals(player.username.clone()))
            .with(user::first_user_game::fetch())
            .with(user::second_user_game::fetch())
            .exec()
            .await
            .unwrap()
            .unwrap();
        self.time.unwrap_or(1) != 0
            && player.rating > self.rating_min
            && player.rating < self.rating_max
            && user.first_user_game().unwrap().is_none()
            && user.second_user_game().unwrap().is_none()
    }

    pub async fn create_or_join(
        &self,
        client: &web::Data<PrismaClient>,
        game_key: &str,
        player: &MatchPlayer,
    ) -> Result<game::Data, ()> {

        if !self.validate(client, player).await {
            return Err(());
        }
        Ok(match self.match_if_possible(
            &client,
            &game_key,
            &player,
        ).await {
            Some(g) => g,
            None => self.create_game(
                &client,
                &game_key,
                &player,
            ).await,
        })
    }

    // method to add a game to table from this game request
    pub async fn create_game(
        &self,
        client: &web::Data<PrismaClient>,
        game_key: &str,
        player: &MatchPlayer,
    ) -> game::Data {
        let alphabet: [char; 62] = [
            '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];
        let mut id: String;
        loop {
            id = nanoid!{6, &alphabet};
            if client
                .game()
                .find_unique(game::id::equals(id.clone()))
                .exec()
                .await
                .unwrap()
                .is_none()
            {
                break;
            }
        }

        client
            .game()
            .create(
                id,
                self.rated,
                game_key.to_string(),
                player.rating,
                self.rating_min,
                self.rating_max,
                "".to_string(),
                0,
                GameStatus::Waiting.to_string(),
                vec![
                    game::clock_initial::set(self.time),
                    game::clock_increment::set(self.increment),
                    game::first_time::set(self.time),
                    game::second_time::set(self.time),
                    game::start_pos::set(self.start_pos.clone()),
                    if player.first {
                        game::first_user::connect(user::username::equals(player.username.clone()))
                    } else {
                        game::second_user::connect(user::username::equals(player.username.clone()))
                    },
                ],
            )
            .exec()
            .await
            .unwrap()
    }

    // method to match player with an existing game if criteria are met
    pub async fn match_if_possible(
        &self,
        client: &web::Data<PrismaClient>,
        game_key: &str,
        player: &MatchPlayer,
    ) -> Option<game::Data> {
        let games: Vec<game::Data> = client
            .game()
            .find_many(vec![
                game::game_key::equals(game_key.to_string()),
                game::clock_initial::equals(self.time),
                game::clock_increment::equals(self.increment),
                if player.first {
                    game::first_username::equals(None)
                } else {
                    game::second_username::equals(None)
                },
            ])
            .exec()
            .await
            .unwrap();

        let filtered_games = games.iter().filter(|g| {
            player.rating_min < g.rating
                && player.rating_max > g.rating
                && g.rating_min < player.rating
                && g.rating_max > player.rating
        });

        if filtered_games.clone().count() == 0 {
            return None;
        }

        let game = filtered_games.min_by_key(|g| g.created_at).unwrap();

        Some(
            client
                .game()
                .update(
                    game::id::equals(game.id.clone()),
                    vec![
                        if player.first {
                            game::first_user::connect(user::username::equals(player.username.clone()))
                        } else {
                            game::second_user::connect(user::username::equals(player.username.clone()))
                        },
                        game::status::set(GameStatus::Started.to_string()),
                    ],
                )
                .exec()
                .await
                .unwrap(),
        )
    }
}

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
                moves: self.moves.clone(),
                first_time: self.first_time,
                second_time: self.second_time,
                status: GameStatus::from_str(&self.status),
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
