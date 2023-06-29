use actix_web::web;

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
}
