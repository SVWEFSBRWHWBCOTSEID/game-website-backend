use rand::Rng;

use crate::{models::{general::{MatchPlayer, Profile, Country, Side}, res::CreateUserResponse, req::CreateGameReq}, prisma::user, common::CustomError};
use super::perf::PerfVec;


impl user::Data {
    // method to get provisional for game
    pub fn get_provisional(&self, game_key: &str) -> Result<bool, CustomError> {
        let perfs = self.perfs().or(Err(CustomError::InternalError))?;

        Ok(perfs.iter().find(|p| p.game_key == game_key).ok_or(CustomError::InternalError)?.prov)
    }

    // method to get rating for game
    pub fn get_rating(&self, game_key: &str) -> Result<i32, CustomError> {
        let perfs = self.perfs().or(Err(CustomError::InternalError))?;

        Ok(perfs.iter().find(|p| p.game_key == game_key).ok_or(CustomError::InternalError)?.rating)
    }

    // method to construct response from prisma user struct
    pub fn to_create_user_res(&self) -> Result<CreateUserResponse, CustomError> {
        let perfs = self.perfs().or(Err(CustomError::InternalError))?;

        Ok(CreateUserResponse {
            username: self.username.clone(),
            created_at: self.created_at.to_string(),
            perfs: perfs.to_perfs_struct()?,
            profile: Profile {
                country: Country::from_str(&self.country),
                location: self.location.clone(),
                bio: self.bio.clone(),
                first_name: self.first_name.clone(),
                last_name: self.last_name.clone(),
            },
            url: self.url.clone(),
            playing: self.playing.clone(),
        })
    }

    pub fn to_match_player(&self, game_key: &str, req: &CreateGameReq) -> MatchPlayer {

        let mut rng = rand::thread_rng();

        MatchPlayer {
            username: self.username.clone(),
            provisional: self.get_provisional(game_key).unwrap(),
            rating: self.get_rating(game_key).unwrap(),
            rating_min: req.rating_min,
            rating_max: req.rating_max,
            first: match req.side {
                Side::First => true,
                Side::Second => false,
                Side::Random => rng.gen_range(0..1) == 0,
            },
        }
    }
}
