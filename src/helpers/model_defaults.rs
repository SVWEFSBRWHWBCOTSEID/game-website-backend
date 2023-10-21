use crate::models::general::{GamePerf, Profile, Country, ClockPreferences, GamePreferences, TenthSeconds, Preferences};
use crate::common::WebErr;


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
            image_url: None,
        }
    }
}

impl Default for Preferences {
    fn default() -> Self {
        Preferences {
            clock: ClockPreferences::default(),
            game: GamePreferences::default(),
        }
    }
}

impl Default for ClockPreferences {
    fn default() -> Self {
        ClockPreferences {
            show_tenth_seconds: TenthSeconds::Critical,
            show_progress_bars: true,
            play_critical_sound: true,
        }
    }
}

impl Default for GamePreferences {
    fn default() -> Self {
        GamePreferences {
            confirm_resign: true,
            board_scroll: true,
        }
    }
}
