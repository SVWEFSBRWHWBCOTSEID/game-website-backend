use std::str::FromStr;
use parking_lot::Mutex;
use actix_session::Session;
use actix_web::{post, HttpRequest, web::Data, HttpResponse};

use crate::helpers::general::{get_username, time_millis, set_user_playing, get_game_with_relations, set_user_can_start_game};
use crate::hourglass::Hourglass;
use crate::models::general::{EndType, Offer, MoveOutcome};
use crate::player_stats::PlayerStats;
use crate::prisma::{PrismaClient, game};
use crate::sse::Broadcaster;
use crate::common::WebErr;
use crate::lumber_mill::LumberMill;
use crate::models::{general::GameStatus, res::OK_RES};
use crate::models::events::{GameEvent, GameStateEvent, GameEventType};


// route for adding a move to a game
#[post("/api/game/{id}/move/{move}")]
pub async fn add_move(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
    broadcaster: Data<Mutex<Broadcaster>>,
    hourglass: Data<Mutex<Hourglass>>,
    mill: Data<Mutex<LumberMill>>,
    player_stats: Data<Mutex<PlayerStats>>,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let new_move: String = req.match_info().get("move").unwrap().parse().unwrap();
    let game = get_game_with_relations(&client, &game_id).await?;

    // make sure it is this player's turn and that move is legal
    let first_to_move = game.num_moves() % 2 == 0;
    if first_to_move && game.first_username.clone().unwrap() != username ||
        !first_to_move && game.second_username.clone().unwrap() != username ||
        !mill.lock().validate_move(&game, &new_move)?
    {
        return Err(WebErr::Forbidden(format!("new move is invalid or not player's turn")));
    }

    if game.first_time.is_some() && game.get_moves_vec().len() > 1 {
        hourglass.lock().set_hourglass(
            game_id.clone(),
            if first_to_move {
                game.second_username.clone().unwrap()
            } else {
                game.first_username.clone().unwrap()
            },
            if first_to_move {
                game.second_time.clone().unwrap()
            } else {
                game.first_time.clone().unwrap()
            },
        );
    }

    // Update the board with the new move and get the new board status
    let move_outcome = mill.lock().update_and_check(&game, &new_move, first_to_move)?;
    let move_status = match move_outcome {
        MoveOutcome::None => GameStatus::Started,
        MoveOutcome::FirstWin => GameStatus::FirstWon,
        MoveOutcome::SecondWin => GameStatus::SecondWon,
        _ => GameStatus::Draw,
    };

    let rating_diffs = game.get_rating_diffs(move_status)?;

    broadcaster.lock().game_send(&game_id, GameEvent::GameStateEvent(GameStateEvent {
        r#type: GameEventType::GameState,
        ftime: game.get_new_first_time()?,
        stime: game.get_new_second_time()?,
        moves: vec![new_move.clone()],
        status: move_status,
        end_type: match move_outcome {
            MoveOutcome::FirstWin | MoveOutcome::SecondWin => Some(EndType::Normal),
            MoveOutcome::Draw => Some(EndType::Stalemate),
            _ => None,
        },
        draw_offer: match move_outcome {
            MoveOutcome::None => Offer::from_str(&game.draw_offer)?,
            _ => Offer::None,
        },
        frating_diff: rating_diffs.0,
        srating_diff: rating_diffs.1,
    }));

    if move_outcome != MoveOutcome::None {
        set_user_playing(&client, &game.first_username.clone().unwrap(), None).await?;
        set_user_playing(&client, &game.second_username.clone().unwrap(), None).await?;
        set_user_can_start_game(&client, &game.first_username.clone().unwrap(), true).await?;
        set_user_can_start_game(&client, &game.second_username.clone().unwrap(), true).await?;
        game.update_ratings(&client, move_status).await?;

        player_stats.lock().update_games(-1, &broadcaster.lock());
        mill.lock().boards.remove(&game.id);
    }

    client
        .game()
        .update(
            game::id::equals(game_id.clone()),
            vec![
                game::moves::set({
                    let mut moves = game.moves.clone();
                    match moves.len() {
                        0 => moves.push_str(&new_move),
                        _ => {
                            moves.push_str(" ");
                            moves.push_str(&new_move);
                        },
                    }
                    moves
                }),
                game::first_time::set(game.get_new_first_time()?),
                game::second_time::set(game.get_new_second_time()?),
                game::last_move_time::set(time_millis()),
                game::status::set(match move_outcome {
                    MoveOutcome::None => GameStatus::Started,
                    MoveOutcome::FirstWin => GameStatus::FirstWon,
                    MoveOutcome::SecondWin => GameStatus::SecondWon,
                    _ => GameStatus::Draw,
                }.to_string()),
                game::win_type::set(match move_outcome {
                    MoveOutcome::FirstWin | MoveOutcome::SecondWin => Some(EndType::Normal.to_string()),
                    MoveOutcome::Draw => Some(EndType::Stalemate.to_string()),
                    _ => None,
                }),
                game::draw_offer::set(match move_outcome {
                    MoveOutcome::None => game.draw_offer.clone(),
                    _ => Offer::None.to_string(),
                }),
            ],
        )
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error updating game with id {} to add move", game_id))))?;

    Ok(HttpResponse::Ok().json(OK_RES))
}
