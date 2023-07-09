use actix_web::web;

use crate::models::general::{GamePerf, Perfs};
use crate::prisma::{user, PrismaClient};
use crate::models::req::CreateUserReq;


impl CreateUserReq {
    // method to check that this username does not already exist
    pub async fn validate(&self, client: &web::Data<PrismaClient>) -> bool {
        client
            .user()
            .find_unique(user::username::equals(self.username.clone()))
            .exec()
            .await
            .unwrap()
            .is_none()
    }

    // method to add a user to table from this user request
    pub async fn create_user(&self, client: &web::Data<PrismaClient>) -> user::Data {

        let hashed_pass = bcrypt::hash(&self.password, bcrypt::DEFAULT_COST).unwrap();

        let starting_perf = GamePerf {
            games: 0,
            rating: 1500,
            rd: 500.0,
            prog: 0,
            prov: true,
        };
        let perfs = Perfs {
            ttt: starting_perf,
            uttt: starting_perf,
            c4: starting_perf,
            pc: starting_perf,
        };

        client
            .user()
            .create(
                self.username.clone(),
                hashed_pass,
                serde_json::to_string(&perfs).unwrap(),
                "Empty".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "http://localhost:3000/user/".to_string() + &self.username,
                vec![],
            )
            .exec()
            .await
            .unwrap()
    }
}
