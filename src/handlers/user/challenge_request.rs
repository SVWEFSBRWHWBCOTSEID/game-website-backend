use parking_lot::Mutex;
use actix_session::Session;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, HttpRequest, post};

use crate::common::WebErr;
use crate::helpers::create_game::join_game;
use crate::helpers::general::{get_username, gen_nanoid, get_user_with_relations, set_user_can_start_game};
use crate::models::events::{UserEvent, UserEventType, ChallengeEvent, ChallengeDeclinedEvent, ChallengeCanceledEvent};
use crate::models::general::{Offer, GameStatus, Side, GameKey, GameType, TimeControl, Challenge};
use crate::models::req::ChallengeReq;
use crate::models::res::OK_RES;
use crate::player_stats::PlayerStats;
use crate::prisma::{PrismaClient, user, game, challenge};
use crate::sse::Broadcaster;


// route for sending challenge request or accepting/declining an incoming challenge request
#[post("/api/challenge/{username}/{accept}")]
pub async fn challenge_request(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
    data: Option<Json<ChallengeReq>>,
    broadcaster: Data<Mutex<Broadcaster>>,
    player_stats: Data<Mutex<PlayerStats>>,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let opponent: String = req.match_info().get("username").unwrap().parse().unwrap();
    let accept: bool = req.match_info().get("accept").unwrap().parse().unwrap();

    let user = get_user_with_relations(&client, &username).await?;

    if accept && !user.can_start_game {
        return Err(WebErr::BadReq(format!("user {} cannot accept or send challenge (can_start_game is false)", username)));
    }

    // If `username` has already sent a challenge to `opponent`
    if let Some(existing) = client
        .challenge()
        .find_unique(challenge::username_opponent_name(username.clone(), opponent.clone()))
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error searching for challenge for user {}", username))))?
    {
        if accept {
            return Err(WebErr::BadReq(format!("user {} already sent a challenge request to {}", username, opponent)));
        }

        // Cancel the challenge if the original challenger sends `false`
        client
            .game()
            .delete(game::id::equals(existing.game_id.clone()))
            .exec()
            .await
            .or(Err(WebErr::Internal(format!("error deleting challenge game with id {}", existing.game_id))))?;

        set_user_can_start_game(&client, &opponent, true).await?;

        broadcaster.lock().user_send(&opponent.clone(), UserEvent::ChallengeCanceledEvent(ChallengeCanceledEvent {
            r#type: UserEventType::ChallengeCanceled,
            opponent: opponent.clone(),
        }));

        return Ok(HttpResponse::Ok().json(OK_RES));
    }

    // If `opponent` has already sent a challenge to `username`
    if let Some(existing) = client
        .challenge()
        .find_unique(challenge::username_opponent_name(opponent.clone(), username.clone()))
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error searching for challenge for user {}", username))))?
    {
        if accept {
            // Start the game if the `opponent` sends `true`
            let game = client
                .game()
                .find_unique(game::id::equals(existing.game_id.clone()))
                .exec()
                .await
                .or(Err(WebErr::Internal(format!("error fetching challenge game with id {}", existing.game_id))))?
                .ok_or(WebErr::NotFound(format!("could not find challenge game with id {}", existing.game_id)))?;

            let perf = user.perfs().unwrap().iter().find(|p| p.game_key == game.game_key.to_string()).unwrap();
            join_game(&client, &game, game.first_username.is_none(), username.clone(), perf.rating as i32, perf.prov, &broadcaster, &player_stats).await?;
        } else {
            // Decline the challenge if the `opponent` sends `false`
            client
                .game()
                .delete(game::id::equals(existing.game_id.clone()))
                .exec()
                .await
                .or(Err(WebErr::Internal(format!("error deleting challenge game with id {}", existing.game_id))))?;

            set_user_can_start_game(&client, &opponent, true).await?;

            broadcaster.lock().user_send(&opponent.clone(), UserEvent::ChallengeDeclinedEvent(ChallengeDeclinedEvent {
                r#type: UserEventType::ChallengeDeclined,
                opponent: opponent.clone(),
            }));
        }

        return Ok(HttpResponse::Ok().json(OK_RES));
    }

    // If no challenge exists, create one
    if !accept {
        return Err(WebErr::BadReq(format!("cannot decline challenge from user {} -- challenge doesn't exist", opponent)));
    }

    let challenge_req = data
        .unwrap_or(Err(WebErr::BadReq(format!("new challenge request missing json body")))?)
        .into_inner();
    let game_id = gen_nanoid(&client).await;

    let game = client
        .game()
        .create(
            game_id,
            challenge_req.rated,
            challenge_req.game_key.to_string(),
            0,
            0,
            "".to_string(),
            0,
            GameStatus::Waiting.to_string(),
            Offer::None.to_string(),
            Offer::None.to_string(),
            challenge_req.side == Side::Random,
            false,
            vec![
                game::clock_initial::set(challenge_req.time),
                game::clock_increment::set(challenge_req.increment),
                game::first_time::set(challenge_req.time),
                game::second_time::set(challenge_req.time),
                game::start_pos::set(challenge_req.start_pos.clone()),
                if challenge_req.side == Side::First {
                    game::first_user::connect(user::username::equals(username.clone()))
                } else {
                    game::second_user::connect(user::username::equals(username.clone()))
                },
                if challenge_req.side == Side::First {
                    game::first_rating::set(Some(user.get_rating(&challenge_req.game_key.to_string())? as i32))
                } else {
                    game::second_rating::set(Some(user.get_rating(&challenge_req.game_key.to_string())? as i32))
                },
                if challenge_req.side == Side::First {
                    game::first_prov::set(Some(user.get_provisional(&challenge_req.game_key.to_string())?))
                } else {
                    game::second_prov::set(Some(user.get_provisional(&challenge_req.game_key.to_string())?))
                },
            ],
        )
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error creating game for user {}'s challenge request", username))))?;

    let challenge = client
        .challenge()
        .create(
            user::username::equals(username.clone()),
            user::username::equals(opponent.clone()),
            game::id::equals(game.id.clone()),
            vec![],
        )
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error creating challenge request for user {}", username))))?;

    set_user_can_start_game(&client, &username, false).await?;

    broadcaster.lock().user_send(&opponent.clone(), UserEvent::ChallengeEvent(ChallengeEvent {
        r#type: UserEventType::Challenge,
        challenge: Challenge {
            user: user.to_player(&challenge_req.game_key.to_string())?,
            game: GameType {
                key: challenge_req.game_key.to_string(),
                name: GameKey::get_game_name(&challenge_req.game_key.to_string())?,
            },
            id: game.id,
            rated: challenge_req.rated,
            side: challenge_req.side,
            time_control: TimeControl {
                initial: challenge_req.time,
                increment: challenge_req.increment,
            },
            created_at: challenge.created_at.to_string(),
        },
    }));

    Ok(HttpResponse::Ok().json(OK_RES))
}
