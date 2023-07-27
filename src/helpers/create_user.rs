use std::env;
use actix_web::web;
use strum::IntoEnumIterator;

use crate::common::WebErr;
use crate::models::general::{GameKey, GamePerf, Profile};
use crate::prisma::{user, PrismaClient, perf};
use crate::models::req::CreateUserReq;
use super::general::get_user_with_relations;


impl CreateUserReq {
    // method to check that this username does not already exist
    pub async fn validate(&self, client: &web::Data<PrismaClient>) -> Result<bool, WebErr> {
        Ok(client
            .user()
            .find_unique(user::username::equals(self.username.clone()))
            .exec()
            .await
            .or(Err(WebErr::Internal(format!("error trying to fetch user {}", self.username))))?
            .is_none())
    }

    // method to add a user to table from this user request
    pub async fn create_user(&self, client: &web::Data<PrismaClient>) -> Result<user::Data, WebErr> {

        let hashed_pass = bcrypt::hash(&self.password, bcrypt::DEFAULT_COST).unwrap();

        client
            .user()
            .create(
                self.username.clone(),
                hashed_pass,
                false,
                Profile::default().country.to_string(),
                Profile::default().location,
                Profile::default().bio,
                Profile::default().first_name,
                Profile::default().last_name,
                [env::var("DOMAIN").unwrap(), "/profile/".to_string(), self.username.clone()].concat(),
                vec![],
            )
            .exec()
            .await
            .or(Err(WebErr::Internal(format!("error creating user {}", self.username))))?;

        client
            .perf()
            .create_many(
                GameKey::iter().map(|k|
                    perf::create_unchecked(
                        self.username.clone(),
                        k.to_string(),
                        GamePerf::default().games,
                        GamePerf::default().rating,
                        GamePerf::default().rd as f64,
                        GamePerf::default().prog,
                        GamePerf::default().prov,
                        vec![],
                    )
                ).collect()
            )
            .exec()
            .await
            .or(Err(WebErr::Internal(format!("error creating perfs for user {}", self.username))))?;

        let new_user = get_user_with_relations(client, &self.username).await?;

        Ok(new_user)
    }
}
