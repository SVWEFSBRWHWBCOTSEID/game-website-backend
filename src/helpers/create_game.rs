use std::env;
use parking_lot::Mutex;
use actix_web::web;

use crate::common::WebErr;
use crate::models::events::{UserEvent, GameStartEvent, UserEventType};
use crate::models::general::{GameStatus, MatchPlayer, GameKey, Offer};
use crate::models::req::CreateGameReq;
use crate::prisma::PrismaClient;
use crate::prisma::{game, user};
use crate::sse::Broadcaster;
use super::general::{set_user_playing, send_lobby_event, gen_game_nanoid};


impl CreateGameReq {
    // method to validate this game request
    pub async fn validate(
        &self, 
        client: &web::Data<PrismaClient>,
        player: &MatchPlayer,
    ) -> Result<bool, WebErr> {
        let user = client
            .user()
            .find_unique(user::username::equals(player.username.clone()))
            .with(user::first_user_games::fetch(vec![]))
            .with(user::second_user_games::fetch(vec![]))
            .exec()
            .await
            .or(Err(WebErr::Internal(format!("could not find user {}", player.username))))?
            .unwrap();
        Ok(self.time.unwrap_or(1) != 0
            && player.rating > self.rating_min
            && player.rating < self.rating_max
            && user.playing == None)
    }

    pub async fn create_or_join(
        &self,
        client: &web::Data<PrismaClient>,
        game_key: &str,
        player: &MatchPlayer,
        broadcaster: &web::Data<Mutex<Broadcaster>>,
    ) -> Result<game::Data, WebErr> {

        if !self.validate(client, player).await? {
            return Err(WebErr::Forbidden(format!("user {} does not meet requirements to join this game", player.username)));
        }
        Ok(match self.match_if_possible(
            client,
            game_key,
            player,
            broadcaster,
        ).await? {
            Some(g) => g,
            None => self.create_game(
                client,
                game_key,
                player,
                broadcaster,
            ).await?,
        })
    }

    // method to add a game to table from this game request
    pub async fn create_game(
        &self,
        client: &web::Data<PrismaClient>,
        game_key: &str,
        player: &MatchPlayer,
        broadcaster: &web::Data<Mutex<Broadcaster>>,
    ) -> Result<game::Data, WebErr> {
        let id = gen_game_nanoid(client).await;

        let game = client
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
                Offer::None.to_string(),
                Offer::None.to_string(),
                player.random,
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
            .or(Err(WebErr::Internal(format!("error creating game"))))?;

        send_lobby_event(&client, &broadcaster).await?;

        Ok(game)
    }

    // method to match player with an existing game if criteria are met
    pub async fn match_if_possible(
        &self,
        client: &web::Data<PrismaClient>,
        game_key: &str,
        player: &MatchPlayer,
        broadcaster: &web::Data<Mutex<Broadcaster>>,
    ) -> Result<Option<game::Data>, WebErr> {
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
            .or(Err(WebErr::Internal(format!("error fetching games"))))?;

        let filtered_games = games.iter().filter(|g| {
            player.rating_min < g.rating
                && player.rating_max > g.rating
                && g.rating_min < player.rating
                && g.rating_max > player.rating
        });
        if filtered_games.clone().count() == 0 {
            return Ok(None);
        }
        let game = filtered_games.min_by_key(|g| g.created_at).unwrap();

        set_user_playing(&client, &player.username, Some([env::var("DOMAIN").unwrap(), "/game/".to_string(), game.id.clone()].concat())).await?;
        set_user_playing(
            &client,
            if player.first {
                game.second_username.as_ref().unwrap()
            } else {
                game.first_username.as_ref().unwrap()
            },
            Some([env::var("DOMAIN").unwrap(), "/game/".to_string(), game.id.clone()].concat()),
        ).await?;

        broadcaster.lock().user_send(&player.username, UserEvent::GameStartEvent(GameStartEvent {
            r#type: UserEventType::GameStart,
            game: GameKey::from_str(game_key)?,
            id: game.id.clone(),
        }));
        broadcaster.lock().user_send(if player.first {
            game.second_username.as_ref().unwrap()
        } else {
            game.first_username.as_ref().unwrap()
        }, UserEvent::GameStartEvent(GameStartEvent {
            r#type: UserEventType::GameStart,
            game: GameKey::from_str(game_key)?,
            id: game.id.clone(),
        }));

        let updated_game = client
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
            .or(Err(WebErr::Internal(format!("error updating game with id {}", game.id))))?;

        send_lobby_event(&client, &broadcaster).await?;

        Ok(Some(updated_game))
    }
}
