use std::str::FromStr;
use actix_web::web;

use crate::get_key_name;
use crate::models::general::{GameType, Clock, GameState, Player, MatchPlayer, GameStatus};
use crate::prisma::{game, user};
use crate::prisma::PrismaClient;
use crate::models::req::CreateGameReq;
use crate::models::res::GameResponse;


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
        client: &web::Data<PrismaClient>,
        game_key: &str,
        player: &MatchPlayer,
    ) -> game::Data {

        let first_name: Option<String>;
        let second_name: Option<String>;

        if player.first {
            first_name = Some(player.name.clone());
            second_name = None;
        } else {
            first_name = None;
            second_name = Some(player.name.clone());
        }

        client
            .game()
            .create(
                self.rated,
                game_key.to_string(),
                player.rating,
                self.rating_min,
                self.rating_max,
                "".to_string(),
                "Waiting".to_string(),
                vec![
                    game::clock_initial::set(self.time),
                    game::clock_increment::set(self.increment),
                    game::first_time::set(self.time),
                    game::second_time::set(self.time),
                    game::first_username::set(first_name),
                    game::second_username::set(second_name),
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
                if player.first { game::first_username::equals(None) }
                else { game::second_username::equals(None) },
            ])
            .exec()
            .await
            .unwrap();

        let filtered_games = games
            .iter()
            .filter(|g| {
                player.rating_min < g.rating && player.rating_max > g.rating &&
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
                    if player.first { game::first_username::set(Some(player.name.clone())) }
                    else { game::second_username::set(Some(player.name.clone())) },
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
    pub async fn to_game_res(&self, client: &web::Data<PrismaClient>) -> GameResponse {

        GameResponse {
            id: self.id.clone(),
            created_at: self.created_at.to_string(),
            rated: self.rated,
            game: GameType {
                key: self.game_key.clone(),
                name: get_key_name(&self.game_key).unwrap(),
            },
            clock: Clock {
                initial: self.clock_initial,
                increment: self.clock_increment,
            },
            first_player: match &self.first_username {
                Some(n) => {
                    let user = client
                        .user()
                        .find_unique(user::username::equals(n.clone()))
                        .exec()
                        .await
                        .unwrap()
                        .unwrap();
                    Some(Player {
                        name: self.first_username.clone().unwrap(),
                        provisional: user.get_provisional(&self.game_key).unwrap(),
                        rating: user.get_rating(&self.game_key).unwrap(),
                    })
                },
                None => None,
            },
            second_player: match &self.second_username {
                Some(n) => {
                    let user = client
                        .user()
                        .find_unique(user::username::equals(n.clone()))
                        .exec()
                        .await
                        .unwrap()
                        .unwrap();
                    Some(Player {
                        name: self.second_username.clone().unwrap(),
                        provisional: user.get_provisional(&self.game_key).unwrap(),
                        rating: user.get_rating(&self.game_key).unwrap(),
                    })
                },
                None => None,
            },
            start_pos: self.start_pos.clone(),
            game_state: GameState {
                moves: self.moves.clone(),
                first_time: self.first_time,
                second_time: self.second_time,
                status: GameStatus::from_str(&self.status).unwrap(),
            },
        }
    }
}
