use actix_web::web;

use crate::get_key_name;
use crate::models::general::{GameType, Clock, GameState, Player, MatchPlayer};
use crate::prisma::game;
use crate::prisma::{GameStatus, PrismaClient};
use crate::models::req::{CreateGameReq};
use crate::models::res::{GameResponse};


impl CreateGameReq {
    // method to validate this game request
    pub fn validate(&self, player: &MatchPlayer) -> bool {
        self.time.unwrap_or(1) != 0 &&
        player.rating > self.rating_min &&
        player.rating < self.rating_max
    }

    // method to add a game to table from this game request
    pub async fn create_game(
        &self,
        client: web::Data<PrismaClient>,
        game_key: &str,
        player: &MatchPlayer,
    ) -> game::Data {

        let first_name: Option<String>;
        let first_provisional: Option<bool>;
        let first_rating: Option<i32>;
        let second_provisional: Option<bool>;
        let second_name: Option<String>;
        let second_rating: Option<i32>;

        if player.first {
            first_name = Some(player.name.clone());
            first_provisional = Some(player.provisional);
            first_rating = Some(player.rating);
            second_name = None;
            second_provisional = None;
            second_rating = None;
        } else {
            first_name = None;
            first_provisional = None;
            first_rating = None;
            second_name = Some(player.name.clone());
            second_provisional = Some(player.provisional);
            second_rating = Some(player.rating);
        }

        client
            .game()
            .create(
                self.rated,
                game_key.to_string(),
                get_key_name(game_key).unwrap(),
                self.rating_min,
                self.rating_max,
                "".to_string(),
                GameStatus::Waiting,
                vec![
                    game::clock_initial::set(self.time),
                    game::clock_increment::set(self.increment),
                    game::first_time::set(self.time),
                    game::second_time::set(self.time),
                    game::first_name::set(first_name),
                    game::first_provisional::set(first_provisional),
                    game::first_rating::set(first_rating),
                    game::second_name::set(second_name),
                    game::second_provisional::set(second_provisional),
                    game::second_rating::set(second_rating),
                    game::start_pos::set(self.start_pos.clone()),
                ],
            )
            .exec()
            .await
            .unwrap()
    }

    // method to match player with an existing game if criteria are met
    pub async fn match_if_possible(
        &self,
        client: web::Data<PrismaClient>,
        game_key: &str,
        player: &MatchPlayer,
    ) -> Option<game::Data> {

        let games: Vec<game::Data> = client
            .game()
            .find_many(vec![
                game::game_key::equals(game_key.to_string()),
                game::clock_initial::equals(self.time),
                game::clock_increment::equals(self.increment),
                if player.first { game::first_name::equals(None) }
                else { game::second_name::equals(None) },
            ])
            .exec()
            .await
            .unwrap();

        let filtered_games = games
            .iter()
            .filter(|g| {
                let rating = if player.first {
                    g.second_rating.unwrap()
                } else {
                    g.first_rating.unwrap()
                };
                player.rating_min < rating && player.rating_max > rating &&
                g.rating_min < player.rating && g.rating_max > player.rating
            });

        if filtered_games.clone().count() == 0 {
            return None;
        }

        let game = filtered_games
            .min_by_key(|g| g.created_at)
            .unwrap();

        Some(client
            .game()
            .update(
                game::id::equals(game.id.clone()),
                vec![
                    if player.first { game::first_name::set(Some(player.name.clone())) }
                    else { game::second_name::set(Some(player.name.clone())) },
                    if player.first { game::first_provisional::set(Some(player.provisional.clone())) }
                    else { game::second_provisional::set(Some(player.provisional.clone())) },
                    if player.first { game::first_rating::set(Some(player.rating.clone())) }
                    else { game::second_rating::set(Some(player.rating.clone())) },
                ],
            )
            .exec()
            .await
            .unwrap()
        )
    }
}

impl game::Data {
    // method to construct reponse from prisma game struct
    pub fn to_game_res(&self) -> GameResponse {
        
        let game = GameType {
            key: self.game_key.clone(),
            name: self.game_name.clone(),
        };

        let clock = Clock {
            initial: self.clock_initial,
            increment: self.clock_increment,
        };

        let first_player = match self.first_name {
            Some(_) => Some(Player {
                name: self.first_name.clone().unwrap(),
                provisional: self.first_provisional.unwrap(),
                rating: self.first_rating.unwrap(),
            }),
            None => None,
        };
        let second_player = match self.second_name {
            Some(_) => Some(Player {
                name: self.second_name.clone().unwrap(),
                provisional: self.second_provisional.unwrap(),
                rating: self.second_rating.unwrap(),
            }),
            None => None,
        };

        let game_state = GameState {
            moves: self.moves.clone(),
            first_time: self.first_time,
            second_time: self.second_time,
            status: self.status,
        };

        GameResponse {
            id: self.id.clone(),
            created_at: self.created_at.to_string(),
            rated: self.rated,
            game,
            clock,
            first_player,
            second_player,
            start_pos: self.start_pos.clone(),
            game_state,
        }
    }
}
