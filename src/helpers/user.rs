use actix_web::web;
use glicko_2::Tuning;
use rand::Rng;
use strum::IntoEnumIterator;

use crate::models::general::{MatchPlayer, Profile, Country, Side, GameKey, GamePerf};
use crate::models::res::{UserResponse, GameResponse};
use crate::models::req::CreateGameReq;
use crate::prisma::{user, PrismaClient};
use crate::common::WebErr;
use super::general::get_user_with_relations;
use super::perf::PerfVec;


impl user::Data {
    // method to get provisional for game
    pub fn get_provisional(&self, game_key: &str) -> Result<bool, WebErr> {
        let perfs = self.perfs().or(Err(WebErr::Internal(format!("perfs not fetched"))))?;

        Ok(perfs.iter().find(|p| p.game_key == game_key)
            .ok_or(WebErr::Internal(format!("could not find perf for {}", game_key)))?
            .prov)
    }

    // method to get rating for game
    pub fn get_rating(&self, game_key: &str) -> Result<f64, WebErr> {
        let perfs = self.perfs().or(Err(WebErr::Internal(format!("perfs not fetched"))))?;

        Ok(perfs.iter().find(|p| p.game_key == game_key)
            .ok_or(WebErr::Internal(format!("could not find perf for {}", game_key)))?
            .rating)
    }

    // method to get rating deviation for game
    pub fn get_rd(&self, game_key: &str) -> Result<f64, WebErr> {
        let perfs = self.perfs().or(Err(WebErr::Internal(format!("perfs not fetched"))))?;

        Ok(perfs.iter().find(|p| p.game_key == game_key)
            .ok_or(WebErr::Internal(format!("could not find perf for {}", game_key)))?
            .rd)
    }

    // method to get volatility for game
    pub fn get_volatility(&self, game_key: &str) -> Result<f64, WebErr> {
        let perfs = self.perfs().or(Err(WebErr::Internal(format!("perfs not fetched"))))?;

        Ok(perfs.iter().find(|p| p.game_key == game_key)
            .ok_or(WebErr::Internal(format!("could not find perf for {}", game_key)))?
            .volatility)
    }

    // method to get change constraint for game
    pub fn get_tau(&self, game_key: &str) -> Result<f64, WebErr> {
        let perfs = self.perfs().or(Err(WebErr::Internal(format!("perfs not fetched"))))?;

        Ok(perfs.iter().find(|p| p.game_key == game_key)
            .ok_or(WebErr::Internal(format!("could not find perf for {}", game_key)))?
            .tau)
    }

    // method to get glicko 2 tuning struct for game
    pub fn get_tuning(&self, game_key: &str) -> Result<Tuning, WebErr> {
        let perfs = self.perfs().or(Err(WebErr::Internal(format!("perfs not fetched"))))?;

        Ok(Tuning::new(
            perfs.iter().find(|p| p.game_key == game_key)
                .ok_or(WebErr::Internal(format!("could not find perf for {}", game_key)))?
                .rating,
            perfs.iter().find(|p| p.game_key == game_key)
                .ok_or(WebErr::Internal(format!("could not find perf for {}", game_key)))?
                .rd,
            perfs.iter().find(|p| p.game_key == game_key)
                .ok_or(WebErr::Internal(format!("could not find perf for {}", game_key)))?
                .volatility,
            perfs.iter().find(|p| p.game_key == game_key)
                .ok_or(WebErr::Internal(format!("could not find perf for {}", game_key)))?
                .tau,
        ))
    }

    // method to add perfs if user is missing perfs for new games
    pub async fn update_perfs(&mut self, client: &web::Data<PrismaClient>) -> Result<(), WebErr> {
        for k in GameKey::iter() {
            if self.perfs().or(Err(WebErr::Internal(format!("perfs not fetched"))))?.iter()
                .find(|p| p.game_key == k.to_string()).is_none()
            {
                client
                    .perf()
                    .create_unchecked(
                        self.username.clone(),
                        k.to_string(),
                        GamePerf::default().games,
                        GamePerf::default().rating,
                        GamePerf::default().rd,
                        GamePerf::default().volatility,
                        GamePerf::default().tau,
                        GamePerf::default().prog,
                        GamePerf::default().prov,
                        vec![],
                    )
                    .exec()
                    .await
                    .or(Err(WebErr::Internal(format!("error updating perfs"))))?;
            }
        }
        *self = get_user_with_relations(client, &self.username).await?;

        Ok(())
    }

    // method to construct response from prisma user struct
    pub fn to_user_res(&self) -> Result<UserResponse, WebErr> {
        let perfs = self.perfs().or(Err(WebErr::Internal(format!("perfs not fetched"))))?;
        let mut games = self.first_user_games().or(Err(WebErr::Internal(format!("first_user_games not fetched"))))?.clone();
        games.extend(self.second_user_games().or(Err(WebErr::Internal(format!("second_user_games not fetched"))))?.clone());

        Ok(UserResponse {
            username: self.username.clone(),
            created_at: self.created_at.to_string(),
            perfs: perfs.to_perfs_struct()?,
            profile: Profile {
                country: Country::from_str(&self.country)?,
                location: self.location.clone(),
                bio: self.bio.clone(),
                first_name: self.first_name.clone(),
                last_name: self.last_name.clone(),
            },
            url: self.url.clone(),
            playing: self.playing.clone(),
            games: games.iter().map(|g| Ok::<GameResponse, WebErr>(g.to_game_res()?)).flatten().collect(),
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
                Side::Random => rng.gen_range(0..2) == 0,
            },
            random: req.side == Side::Random,
        }
    }
}
