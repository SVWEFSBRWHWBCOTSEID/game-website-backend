use actix_web::web;
use rand::Rng;
use strum::IntoEnumIterator;

use crate::models::general::{MatchPlayer, Profile, Country, Side, GameKey, GamePerf};
use crate::models::res::UserResponse;
use crate::models::req::CreateGameReq;
use crate::prisma::{user, PrismaClient};
use crate::common::CustomError;
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

    // method to add perfs if user is missing perfs for new games
    pub async fn update_perfs(&mut self, client: &web::Data<PrismaClient>) -> Result<(), CustomError> {
        for k in GameKey::iter() {
            if self.perfs().or(Err(CustomError::InternalError))?.iter().find(|p| p.game_key == k.to_string()).is_none() {
                client
                    .perf()
                    .create_unchecked(
                        self.username.clone(),
                        k.to_string(),
                        GamePerf::default().games,
                        GamePerf::default().rating,
                        GamePerf::default().rd as f64,
                        GamePerf::default().prog,
                        GamePerf::default().prov,
                        vec![],
                    )
                    .exec()
                    .await
                    .or(Err(CustomError::InternalError))?;
            }
        }
        *self = client
            .user()
            .find_unique(user::username::equals(self.username.clone()))
            .with(user::perfs::fetch(vec![]))
            .exec()
            .await
            .or(Err(CustomError::InternalError))?
            .unwrap();

        Ok(())
    }

    // method to construct response from prisma user struct
    pub fn to_user_res(&self) -> Result<UserResponse, CustomError> {
        let perfs = self.perfs().or(Err(CustomError::InternalError))?;

        Ok(UserResponse {
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
