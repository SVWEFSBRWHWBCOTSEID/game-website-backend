use std::{collections::HashMap, time::{Duration, SystemTime}};
use actix_web::web::Data;
use parking_lot::Mutex;
use tokio::time::{interval_at, Instant};

use crate::{sse::Broadcaster, helpers::general::timeout_player, prisma::PrismaClient, player_stats::PlayerStats};


pub struct Hourglass {
    hourglasses: HashMap<String, Glass>,
}

pub struct Glass {
    username: String,
    millis: i32,
    sys_time: SystemTime,
}

impl Hourglass {
    pub fn create(
        client: Data<PrismaClient>,
        broadcaster: Data<Mutex<Broadcaster>>,
        player_stats: Data<Mutex<PlayerStats>>,
    ) -> Data<Mutex<Self>> {
        let hourglass = Data::new(Mutex::new(Hourglass::new()));

        Hourglass::tick_hourglasses(hourglass.clone(), client, broadcaster, player_stats);
        hourglass
    }

    fn new() -> Self {
        Hourglass {
            hourglasses: HashMap::new(),
        }
    }

    fn tick_hourglasses(
        hourglass: Data<Mutex<Self>>,
        client: Data<PrismaClient>,
        broadcaster: Data<Mutex<Broadcaster>>,
        player_stats: Data<Mutex<PlayerStats>>,
    ) {
        actix_web::rt::spawn(async move {
            let mut interval = interval_at(Instant::now(), Duration::from_millis(100));
            loop {
                interval.tick().await;
                hourglass.lock().break_empty_hourglasses(&client, &broadcaster, &player_stats).await;
            }
        });
    }

    async fn break_empty_hourglasses(&mut self, client: &Data<PrismaClient>, broadcaster: &Data<Mutex<Broadcaster>>, player_stats: &Data<Mutex<PlayerStats>>) {
        let mut glasses_to_break: Vec<String> = vec![];

        for (game_id, glass) in &mut self.hourglasses {
            if glass.sys_time.elapsed().unwrap().as_millis() as i32 > glass.millis {
                timeout_player(&client, &broadcaster, &player_stats, game_id.to_string(), glass.username.clone()).await.ok();
                glasses_to_break.push(game_id.to_string());
            }
        }
        for id in glasses_to_break {
            self.hourglasses.remove(&id);
        }
    }

    pub fn set_hourglass(&mut self, game_id: String, username: String, millis: i32) {
        log::debug!("game_id: {}, username: {}, millis: {}", game_id, username, millis);
        self.hourglasses.entry(game_id)
            .and_modify(|glass| {
                glass.username = username.clone();
                glass.millis = millis;
                glass.sys_time = SystemTime::now();
            })
            .or_insert(Glass {
                username,
                millis,
                sys_time: SystemTime::now(),
            });
    }
}
