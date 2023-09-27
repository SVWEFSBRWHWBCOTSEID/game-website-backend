use actix_web::web::Data;
use parking_lot::Mutex;

use crate::{sse::Broadcaster, models::events::{LobbyEvent, PlayerStatsEvent, LobbyEventType}};


pub struct PlayerStats {
    pub players: i32,
    pub games: i32,
}

impl PlayerStats {
    pub fn create() -> Data<Mutex<Self>> {
        Data::new(Mutex::new(PlayerStats::new()))
    }

    fn new() -> Self {
        PlayerStats {
            players: 0,
            games: 0,
        }
    }

    fn broadcast(&self, broadcaster: &Broadcaster) {
        broadcaster.lobby_send(LobbyEvent::PlayerStatsEvent(PlayerStatsEvent {
            r#type: LobbyEventType::AllLobbies,
            players: self.players,
            games: self.games,
        }));
    }

    pub fn update_games(&mut self, change: i32, broadcaster: &Broadcaster) {
        self.games += change;
        self.broadcast(broadcaster);
    }

    pub fn set_players(&mut self, new_players: i32, broadcaster: &Broadcaster) {
        if self.players == new_players {
            return;
        }
        self.players = new_players;
        self.broadcast(broadcaster);
    }
}
