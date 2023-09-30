use crate::common::WebErr;
use crate::prisma::challenge;
use crate::models::general::{Challenge, GameKey, GameType, TimeControl, Side};


impl challenge::Data {
    pub fn to_challenge(&self) -> Result<Challenge, WebErr> {
        let game = self.game().or(Err(WebErr::Internal(format!("game relation not fetched"))))?;

        Ok(Challenge {
            username: self.username.clone(),
            opponent: self.opponent_name.clone(),
            id: self.game_id.clone(),
            rated: game.rated,
            game: GameType {
                key: game.game_key.clone(),
                name: GameKey::get_game_name(&game.game_key)?,
            },
            time_control: TimeControl {
                initial: game.clock_initial,
                increment: game.clock_increment,
            },
            side: if game.random_side {
                Side::Random
            } else if game.first_username.is_some() {
                Side::Second
            } else {
                Side::First
            },
            created_at: self.created_at.to_string(),
        })
    }
}
