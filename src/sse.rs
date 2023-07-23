use actix_web::web::{Bytes, Data};
use futures::Stream;
use std::sync::Mutex;
use std::pin::Pin;
use std::time::Duration;
use std::task::{Context, Poll};
use std::collections::HashMap;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::time::{interval_at, Instant};

use crate::common::WebErr;
use crate::models::events::{GameEvent, UserEvent, Event};


pub struct Client(Receiver<Bytes>);

impl Stream for Client {
    type Item = Result<Bytes, WebErr>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_recv(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct Broadcaster {
    user_clients: HashMap<String, Vec<Sender<Bytes>>>,
    game_clients: HashMap<String, Vec<Sender<Bytes>>>,
}

impl Broadcaster {
    pub fn create() -> Data<Mutex<Self>> {
        let broadcaster = Data::new(Mutex::new(Broadcaster::new()));

        Broadcaster::spawn_ping(broadcaster.clone());
        broadcaster
    }

    fn new() -> Self {
        Broadcaster {
            user_clients: HashMap::new(),
            game_clients: HashMap::new(),
        }
    }

    // Heartbeat on 10 second interval
    fn spawn_ping(me: Data<Mutex<Self>>) {
        actix_web::rt::spawn(async move {
            let mut interval = interval_at(Instant::now(), Duration::from_secs(10));
            loop {
                interval.tick().await;
                me.lock().unwrap().remove_stale_clients();
            }
        });
    }

    fn remove_stale_clients(&mut self) {
        for vec in self.user_clients.values_mut() {
            vec.retain(|x| x.clone().try_send(Bytes::from("event: internal_status\ndata: ping\n\n")).is_ok());
        }
        self.user_clients.retain(|_, v| v.len() != 0);

        for vec in self.game_clients.values_mut() {
            vec.retain(|x| x.clone().try_send(Bytes::from("event: internal_status\ndata: ping\n\n")).is_ok());
        }
        self.game_clients.retain(|_, v| v.len() != 0);
    }

    pub fn new_user_client(&mut self, username: String) -> (Client, Sender<Bytes>) {
        let (tx, rx) = channel(100);

        self.user_clients.entry(username)
            .and_modify(|v| v.push(tx.clone()))
            .or_insert(vec![tx.clone()]);
        (Client(rx), tx)
    }

    pub fn new_game_client(&mut self, game_id: String) -> (Client, Sender<Bytes>) {
        let (tx, rx) = channel(100);

        self.game_clients.entry(game_id)
            .and_modify(|v| v.push(tx.clone()))
            .or_insert(vec![tx.clone()]);
        (Client(rx), tx)
    }

    pub fn user_send(&self, username: &str, msg: UserEvent) {
        let msg = Bytes::from(["data: ", &msg.to_string(), "\n\n"].concat());

        for client in self.user_clients.get(username).unwrap().iter() {
            client.clone().try_send(msg.clone()).unwrap_or(());
        }
    }

    pub fn game_send(&self, game_id: &str, msg: GameEvent) {
        let msg = Bytes::from(["data: ", &msg.to_string(), "\n\n"].concat());

        for client in self.game_clients.get(game_id).unwrap().iter() {
            client.clone().try_send(msg.clone()).unwrap_or(());
        }
    }

    pub fn send_single(&self, client: &Sender<Bytes>, msg: Event) {
        let msg = Bytes::from(["data: ", &msg.to_string(), "\n\n"].concat());

        client.try_send(msg.clone()).unwrap_or(());
    }
}
