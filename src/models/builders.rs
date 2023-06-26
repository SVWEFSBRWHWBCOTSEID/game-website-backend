use uuid::Uuid;
use chrono::Utc;
use rand::Rng;

use crate::common::get_key_name;
use crate::models::models::{Seek, Game, GameType, Clock, Player, GameState, GameStatus, Side};


impl Game {
    // method to construct a Game from a Seek and game key
    pub fn from_seek(seek: Seek, game_key: String) -> Self {
        
        let game_type = GameType {
            key: game_key.clone(),
            name: get_key_name(&game_key),
        };
        let clock = Clock {
            initial: seek.time,
            increment: seek.increment,
        };
        let state = GameState {
            moves: Vec::new(),
            first_time: seek.time,
            second_time: seek.time,
            status: GameStatus::Started,
        };

        let mut rng = rand::thread_rng();
        let mut first: Option<Player> = None;
        let mut second: Option<Player> = None;
        match seek.side {
            Side::First => {first = Some(seek.player)},
            Side::Second => {second = Some(seek.player)},
            Side::Random => {
                if rng.gen_range(0.0..1.0) < 0.5 {
                    first = Some(seek.player);
                } else {
                    second = Some(seek.player);
                }
            },
        }

        Game {
            id: Uuid::new_v4(),
            rated: seek.rated,
            game: game_type,
            clock,
            created_at: Utc::now().timestamp_millis() as u64,
            first,
            second,
            start_pos: seek.start_pos,
            state,
        }
    }
}

