use crate::{models::general::{GamePerf, Profile, Country}, common::WebErr};


impl Default for GamePerf {
    fn default() -> Self {
        GamePerf {
            games: 0,
            rating: 1500.0,
            rd: 500.0,
            volatility: 0.06,
            tau: 0.8,
            prog: vec![
                1500.0, 1500.0, 1500.0, 1500.0, 1500.0, 1500.0,
                1500.0, 1500.0, 1500.0, 1500.0, 1500.0, 1500.0,
            ],
            prov: true,
        }
    }
}

impl GamePerf {
    pub fn stringify_prog(prog: Vec<f64>) -> String {
        prog.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")
    }

    pub fn prog_from_str(string: &str) -> Result<Vec<f64>, WebErr> {
        Ok(string
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| Ok::<f64, WebErr>(x.parse::<f64>()
                .or(Err(WebErr::Internal(format!("cannot parse prog string as Vec<f64>"))))?
            ))
            .flatten()
            .collect())
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
