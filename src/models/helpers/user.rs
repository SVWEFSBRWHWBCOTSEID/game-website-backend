use actix_web::web;

use crate::models::general::{GamePerf, Perfs, Profile, Country};
use crate::models::res::UserResponse;
use crate::prisma::{user, PrismaClient};
use crate::models::req::CreateUserReq;

impl CreateUserReq {
    // method to add a user to table from this user request
    pub async fn create_user(&self, client: web::Data<PrismaClient>) -> user::Data {
        client
            .user()
            .create(
                self.name.clone(),
                1500,
                true,
                1500,
                true,
                1500,
                true,
                1500,
                true,
                vec![],
            )
            .exec()
            .await
            .unwrap()
    }
}

impl user::Data {
    // method to get provisional for game
    pub fn get_provisional(&self, game_key: &str) -> Option<bool> {
        match game_key {
            "ttt" => Some(self.ttt_provisional),
            "uttt" => Some(self.uttt_provisional),
            "c4" => Some(self.c_4_provisional),
            "pc" => Some(self.pc_provisional),
            _ => None,
        }
    }

    // method to get rating for game
    pub fn get_rating(&self, game_key: &str) -> Option<i32> {
        match game_key {
            "ttt" => Some(self.ttt_rating),
            "uttt" => Some(self.uttt_rating),
            "c4" => Some(self.c_4_rating),
            "pc" => Some(self.pc_rating),
            _ => None,
        }
    }

    // method to construct response from prisma user struct
    pub fn to_user_res(&self) -> UserResponse {

        let ttt_perf = GamePerf {
            games: 0,
            rating: self.ttt_rating,
            rd: 0,
            prog: 0,
            prov: self.ttt_provisional,
        };
        let uttt_perf = GamePerf {
            games: 0,
            rating: self.uttt_rating,
            rd: 0,
            prog: 0,
            prov: self.uttt_provisional,
        };
        let c4_perf = GamePerf {
            games: 0,
            rating: self.c_4_rating,
            rd: 0,
            prog: 0,
            prov: self.c_4_provisional,
        };
        let pc_perf = GamePerf {
            games: 0,
            rating: self.pc_rating,
            rd: 0,
            prog: 0,
            prov: self.pc_provisional,
        };
        let perfs = Perfs {
            ttt: ttt_perf,
            uttt: uttt_perf,
            c4: c4_perf,
            pc: pc_perf,
        };
        let profile = Profile {
            country: Country::US,
            location: "Test location".to_string(),
            bio: "Test bio".to_string(),
            first_name: "Bepler".to_string(),
            last_name: "Koybe".to_string(),
        };

        UserResponse {
            username: self.name.clone(),
            created_at: self.created_at.to_string(),
            perfs,
            profile,
            url: "Test url".to_string(),
            playing: None,
        }
    }
}
