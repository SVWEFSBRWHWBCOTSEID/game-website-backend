use actix_web::web;
use rand::Rng;

use crate::get_key_name;
use crate::models::general::{GameType, Clock, Player, GameState};
use crate::prisma::game::{Data, self};
use crate::prisma::{GameStatus, Side, PrismaClient};
use crate::models::req::{CreateGameReq};
use crate::models::res::{GameResponse};


impl CreateGameReq {
    // method to construct a Game from a Seek and game key
    pub async fn create_game(
        &self,
        client: web::Data<PrismaClient>,
        game_key: &str,
    ) -> Data {

        let mut rng = rand::thread_rng();
        let mut first_name: Option<String> = None;
        let mut first_provisional: Option<bool> = None;
        let mut first_rating: Option<i32> = None;
        let mut second_provisional: Option<bool> = None;
        let mut second_name: Option<String> = None;
        let mut second_rating: Option<i32> = None;
        match self.side {
            Side::First => {
                first_name = Some(self.player.name.clone());
                first_provisional = Some(self.player.provisional);
                first_rating = Some(self.player.rating);
            },
            Side::Second => {
                second_name = Some(self.player.name.clone());
                second_provisional = Some(self.player.provisional);
                second_rating = Some(self.player.rating);
            },
            Side::Random => {
                if rng.gen_range(0.0..1.0) < 0.5 {
                    first_name = Some(self.player.name.clone());
                    first_provisional = Some(self.player.provisional);
                    first_rating = Some(self.player.rating);
                } else {
                    second_name = Some(self.player.name.clone());
                    second_provisional = Some(self.player.provisional);
                    second_rating = Some(self.player.rating);
                }
            },
        }

        client
            .game()
            .create(
                self.rated,
                game_key.to_string(),
                get_key_name(game_key),
                self.start_pos.clone(),
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
                ],
            )
            .exec()
            .await
            .unwrap()
        
    }
}

impl Data {
    // method to construct a GameResponse from prisma result
    pub fn to_game_res(&self) -> GameResponse {
        
        let game = GameType {
            key: self.game_key.clone(),
            name: self.game_name.clone(),
        };

        let clock = Clock {
            initial: self.clock_initial,
            increment: self.clock_increment,
        };

        let first_player: Option<Player>;
        let second_player: Option<Player>;

        match self.first_name {
            Some(_) => first_player = Some(Player {
                name: self.first_name.clone().unwrap(),
                provisional: self.first_provisional.unwrap(),
                rating: self.first_rating.unwrap(),
            }),
            None => first_player = None,
        }
        match self.second_name {
            Some(_) => second_player = Some(Player {
                name: self.second_name.clone().unwrap(),
                provisional: self.second_provisional.unwrap(),
                rating: self.second_rating.unwrap(),
            }),
            None => second_player = None,
        }

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
