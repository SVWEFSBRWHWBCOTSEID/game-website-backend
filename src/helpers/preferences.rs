use std::str::FromStr;
use crate::common::WebErr;
use crate::models::general::{ClockPreferences, GamePreferences, Preferences, TenthSeconds};
use crate::prisma::preferences;


impl preferences::Data {
    pub fn to_preferences_res(self) -> Result<Preferences, WebErr> {
        Ok(Preferences {
            clock: ClockPreferences {
                show_tenth_seconds: TenthSeconds::from_str(&self.show_tenth_seconds)?,
                show_progress_bars: self.show_progress_bars,
                play_critical_sound: self.play_critical_sound,
            },
            game: GamePreferences {
                confirm_resign: self.confirm_resign,
                board_scroll: self.board_scroll,
            }
        })
    }
}
