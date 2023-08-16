use actix_web::web;
use prisma_client_rust::or;

use crate::{models::general::{Perfs, GamePerf}, common::WebErr};
use crate::prisma::{perf, PrismaClient, game};


pub async fn get_perfs_struct(client: &web::Data<PrismaClient>, perfs: Vec<perf::Data>) -> Result<Perfs, WebErr> {
    let ttt = perfs.iter().find(|p| p.game_key == "ttt")
        .ok_or(WebErr::NotFound(format!("ttt perf not found")))?;
    let uttt = perfs.iter().find(|p| p.game_key == "uttt")
        .ok_or(WebErr::NotFound(format!("uttt perf not found")))?;
    let c4 = perfs.iter().find(|p| p.game_key == "c4")
        .ok_or(WebErr::NotFound(format!("c4 perf not found")))?;
    let pc = perfs.iter().find(|p| p.game_key == "pc")
        .ok_or(WebErr::NotFound(format!("pc perf not found")))?;

    Ok(Perfs {
        ttt: ttt.to_game_perf(client).await?,
        uttt: uttt.to_game_perf(client).await?,
        c4: c4.to_game_perf(client).await?,
        pc: pc.to_game_perf(client).await?,
    })
}

impl perf::Data {
    pub async fn to_game_perf(&self, client: &web::Data<PrismaClient>) -> Result<GamePerf, WebErr> {
        let games = client
            .game()
            .find_many(vec![
                or![
                    game::first_username::equals(Some(self.username.clone())),
                    game::second_username::equals(Some(self.username.clone()))
                ],
                game::game_key::equals(self.game_key.clone()),
            ])
            .exec()
            .await
            .or(Err(WebErr::Internal(format!("error fetching games for perf with id {}", self.game_key))))?;

        Ok(GamePerf {
            games: games.len() as i32,
            rating: self.rating,
            rd: self.rd,
            volatility: self.volatility,
            tau: self.tau,
            prog: GamePerf::prog_from_str(&self.prog)?.iter().sum(),
            prov: self.prov,
        })
    }
}
