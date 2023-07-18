use crate::models::general::GamePerf;


impl Default for GamePerf {
    fn default() -> Self {
        GamePerf {
            games: 0,
            rating: 1500,
            rd: 500.0,
            prog: 0,
            prov: true,
        }
    }
}
