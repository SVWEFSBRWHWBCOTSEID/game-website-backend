use std::{task::{Poll, Context}, pin::Pin, time::Duration};
use actix_web::web::{Bytes, Data};
use futures::Stream;
use std::sync::Mutex;
use tokio::{sync::mpsc::{Sender, channel, Receiver}, time::{Instant, interval_at}};

use crate::common::CustomError;


pub struct Client(Receiver<Bytes>);

impl Stream for Client {
    type Item = Result<Bytes, CustomError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_recv(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct Broadcaster {
    clients: Vec<Sender<Bytes>>,
}

impl Broadcaster {
    pub fn create() -> Data<Mutex<Self>> {
        let broadcaster = Data::new(Mutex::new(Broadcaster::new()));

        Broadcaster::spawn_ping(broadcaster.clone());
        broadcaster
    }

    fn new() -> Self {
        Broadcaster {
            clients: Vec::new()
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
        let mut ok_clients = Vec::new();
        for client in self.clients.iter() {
            let result = client.clone().try_send(Bytes::from("event: internal_status\ndata: ping\n\n"));

            if let Ok(()) = result {
                ok_clients.push(client.clone());
            }
        }
        self.clients = ok_clients;
    }

    pub fn new_client(&mut self) -> Client {
        let (tx, rx) = channel(100);

        tx.clone()
            .try_send(Bytes::from("data: connected\n\n"))
            .unwrap();

        self.clients.push(tx);
        Client(rx)
    }

    pub fn send(&self, msg: &str) {
        let msg = Bytes::from(["data: ", msg, "\n\n"].concat());

        for client in self.clients.iter() {
            client.clone().try_send(msg.clone()).unwrap_or(());
        }
    }
}
