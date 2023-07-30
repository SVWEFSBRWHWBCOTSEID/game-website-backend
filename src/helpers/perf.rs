use crate::{models::general::{Perfs, GamePerf}, common::WebErr, prisma::perf};


pub trait PerfVec {
    fn to_perfs_struct(&self) -> Result<Perfs, WebErr>;
}

impl PerfVec for Vec<perf::Data> {
    fn to_perfs_struct(&self) -> Result<Perfs, WebErr> {
        let perfs = self
            .iter()
            .map(|p| Ok::<(String, GamePerf), WebErr>((p.game_key.clone(), p.to_game_perf()?)));

        Ok(Perfs {
            ttt: perfs.clone().flatten().find(|(k, _p)| k == "ttt")
                .ok_or(WebErr::NotFound(format!("ttt perf not found")))?.1,
            uttt: perfs.clone().flatten().find(|(k, _p)| k == "uttt")
                .ok_or(WebErr::NotFound(format!("uttt perf not found")))?.1,
            c4: perfs.clone().flatten().find(|(k, _p)| k == "c4")
                .ok_or(WebErr::NotFound(format!("c4 perf not found")))?.1,
            pc: perfs.clone().flatten().find(|(k, _p)| k == "pc")
                .ok_or(WebErr::NotFound(format!("pc perf not found")))?.1,
        })
    }
}

impl perf::Data {
    pub fn to_game_perf(&self) -> Result<GamePerf, WebErr> {
        Ok(GamePerf {
            games: self.games,
            rating: self.rating,
            rd: self.rd,
            volatility: self.volatility,
            tau: self.tau,
            prog: GamePerf::prog_from_str(&self.prog)?,
            prov: self.prov,
        })
    }
}
