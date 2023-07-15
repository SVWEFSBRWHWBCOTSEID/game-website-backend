use rand::Rng;

use crate::{models::{general::{Perfs, MatchPlayer, Profile, Country, Side}, res::CreateUserResponse, req::CreateGameReq}, prisma::user, common::CustomError};


impl user::Data {
    // method to get provisional for game
    pub fn get_provisional(&self, game_key: &str) -> Result<bool, CustomError> {

        let perfs: Perfs = serde_json::from_str(&self.perfs).unwrap();

        match game_key {
            "ttt" => Ok(perfs.ttt.prov),
            "uttt" => Ok(perfs.uttt.prov),
            "c4" => Ok(perfs.c4.prov),
            "pc" => Ok(perfs.pc.prov),
            _ => Err(CustomError::NotFound),
        }
    }

    // method to get rating for game
    pub fn get_rating(&self, game_key: &str) -> Result<i32, CustomError> {

        let perfs: Perfs = serde_json::from_str(&self.perfs).unwrap();

        match game_key {
            "ttt" => Ok(perfs.ttt.rating),
            "uttt" => Ok(perfs.uttt.rating),
            "c4" => Ok(perfs.c4.rating),
            "pc" => Ok(perfs.pc.rating),
            _ => Err(CustomError::NotFound),
        }
    }

    // method to construct response from prisma user struct
    pub fn to_create_user_res(&self) -> CreateUserResponse {

        CreateUserResponse {
            username: self.username.clone(),
            created_at: self.created_at.to_string(),
            perfs: serde_json::from_str(&self.perfs).unwrap(),
            profile: Profile {
                country: Country::from_str(&self.country),
                location: self.location.clone(),
                bio: self.bio.clone(),
                first_name: self.first_name.clone(),
                last_name: self.last_name.clone(),
            },
            url: self.url.clone(),
            playing: self.playing.clone(),
        }
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
