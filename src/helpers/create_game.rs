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
use super::general::{set_user_playing, send_lobby_event, gen_nanoid};


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
            && (player.rating as i32) > self.rating_min
            && (player.rating as i32) < self.rating_max
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

        // Try to find a game match; if found, join it. Otherwise, create a new game from the req.
        Ok(match self.find_match(client, game_key, player).await? {
            Some(g) =>
                join_game(client, &g, player.first, player.username.clone(), player.rating as i32, player.provisional, broadcaster).await?,
            None =>
                self.create_game(client, game_key, player, broadcaster).await?,
        })
    }

    // Creates a new game from this create game request.
    pub async fn create_game(
        &self,
        client: &web::Data<PrismaClient>,
        game_key: &str,
        player: &MatchPlayer,
        broadcaster: &web::Data<Mutex<Broadcaster>>,
    ) -> Result<game::Data, WebErr> {
        let id = gen_nanoid(client).await;

        let game = client
            .game()
            .create(
                id,
                self.rated,
                game_key.to_string(),
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
                    if player.first {
                        game::first_rating::set(Some(player.rating as i32))
                    } else {
                        game::second_rating::set(Some(player.rating as i32))
                    },
                    if player.first {
                        game::first_prov::set(Some(player.provisional))
                    } else {
                        game::second_prov::set(Some(player.provisional))
                    },
                ],
            )
            .exec()
            .await
            .or(Err(WebErr::Internal(format!("error creating game"))))?;

        send_lobby_event(&client, &broadcaster).await?;

        Ok(game)
    }

    // Attempts to find an existing match for this create game request.
    pub async fn find_match(
        &self,
        client: &web::Data<PrismaClient>,
        game_key: &str,
        player: &MatchPlayer,
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
            let rating = match g.first_rating {
                Some(r) => r,
                None => g.second_rating.unwrap(),
            };
            player.rating_min < rating
                && player.rating_max > rating
                && g.rating_min < player.rating as i32
                && g.rating_max > player.rating as i32
        });
        if filtered_games.clone().count() == 0 {
            return Ok(None);
        }
        Ok(Some(filtered_games.min_by_key(|g| g.created_at).unwrap().clone()))
    }
}

pub async fn join_game(
    client: &web::Data<PrismaClient>,
    game: &game::Data,
    is_first: bool,
    username: String,
    rating: i32,
    provisional: bool,
    broadcaster: &web::Data<Mutex<Broadcaster>>,
) -> Result<game::Data, WebErr> {

    let updated_game = client
        .game()
        .update(
            game::id::equals(game.id.clone()),
            if is_first {
                vec![
                    game::first_user::connect(user::username::equals(username.clone())),
                    game::first_rating::set(Some(rating)),
                    game::first_prov::set(Some(provisional)),
                    game::status::set(GameStatus::Started.to_string()),
                ]
            } else {
                vec![
                    game::second_user::connect(user::username::equals(username.clone())),
                    game::second_rating::set(Some(rating)),
                    game::second_prov::set(Some(provisional)),
                    game::status::set(GameStatus::Started.to_string()),
                ]
            },
        )
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error updating game with id {}", game.id))))?;

    broadcaster.lock().user_send(&updated_game.first_username.clone().unwrap(), UserEvent::GameStartEvent(GameStartEvent {
        r#type: UserEventType::GameStart,
        game: GameKey::from_str(&updated_game.game_key)?,
        id: game.id.clone(),
    }));
    broadcaster.lock().user_send(&updated_game.second_username.clone().unwrap(), UserEvent::GameStartEvent(GameStartEvent {
        r#type: UserEventType::GameStart,
        game: GameKey::from_str(&updated_game.game_key)?,
        id: game.id.clone(),
    }));

    set_user_playing(&client, &updated_game.first_username.clone().unwrap(), Some([env::var("DOMAIN").unwrap(), "/game/".to_string(), game.id.clone()].concat())).await?;
    set_user_playing(&client, &updated_game.second_username.clone().unwrap(), Some([env::var("DOMAIN").unwrap(), "/game/".to_string(), game.id.clone()].concat())).await?;
    send_lobby_event(&client, &broadcaster).await?;

    Ok(updated_game)
}
