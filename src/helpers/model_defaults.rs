use crate::models::general::{GamePerf, Profile, Country};


impl Default for GamePerf {
    fn default() -> Self {
        GamePerf {
            games: 0,
            rating: 1500.0,
            rd: 500.0,
            volatility: 0.06,
            tau: 0.8,
            prog: 0.0,
            prov: true,
        }
    }
}

impl Default for Profile {
    fn default() -> Self {
        Profile {
            country: Country::Empty,
            location: "".to_string(),
            bio: "".to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
        }
    }
}
