use serde::{Deserialize, Serialize};

use super::general::{GameStatus, TimeControl, Player};


#[derive(Deserialize, Serialize)]
pub enum GameEvent {
    ChatMessageEvent(ChatMessageEvent),
    GameStateEvent(GameStateEvent),
    GameFullEvent(GameFullEvent),
}

impl GameEvent {
    pub fn to_string(&self) -> String {
        match self {
            GameEvent::ChatMessageEvent(e) => serde_json::to_string(e).unwrap(),
            GameEvent::GameStateEvent(e) => serde_json::to_string(e).unwrap(),
            GameEvent::GameFullEvent(e) => serde_json::to_string(e).unwrap(),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum UserEvent {
    
}

impl UserEvent {
    pub fn to_string(&self) -> String {
        match self {
            _ => todo!()
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageEvent {
    pub r#type: GameEventType,
    pub username: String,
    pub text: String,
    pub visibility: Visibility,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStateEvent {
    pub r#type: GameEventType,
    pub ftime: Option<i32>,
    pub stime: Option<i32>,
    pub moves: Vec<String>,
    pub status: GameStatus,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameFullEvent {
    pub r#type: GameEventType,
    pub rated: bool,
    pub time_control: TimeControl,
    pub created_at: String,
    pub first: Player,
    pub second: Player,
    pub chat: Vec<ChatMessage>,
    pub state: GameState,
}

#[derive(Deserialize, Serialize)]
pub struct GameState {
    pub ftime: Option<i32>,
    pub stime: Option<i32>,
    pub moves: Vec<String>,
    pub status: GameStatus,
}

#[derive(Deserialize, Serialize)]
pub struct ChatMessage {
    pub username: String,
    pub text: String,
    pub visibility: Visibility,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserEventType {
    GameStart,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GameEventType {
    ChatMessage,
    GameState,
    GameFull,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Visibility {
    Player,
    Spectator,
    Team1,
    Team2,
}
