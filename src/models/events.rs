use serde::{Deserialize, Serialize};

use super::general::{GameStatus, TimeControl, Player};


#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GameEvent {
    ChatMessageEvent(ChatMessageEvent),
    GameStateEvent(GameStateEvent),
    GameFullEvent(GameFullEvent),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum UserEvent {
    
}

#[derive(Deserialize, Serialize)]
pub struct ChatMessageEvent {
    pub r#type: EventType,
    pub username: String,
    pub text: String,
    pub visibility: Visibility,
}

#[derive(Deserialize, Serialize)]
pub struct GameStateEvent {
    pub r#type: EventType,
    pub ftime: Option<i32>,
    pub stime: Option<i32>,
    pub r#move: String,
    pub status: GameStatus,
}

#[derive(Deserialize, Serialize)]
pub struct GameFullEvent {
    pub r#type: EventType,
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
pub enum EventType {
    GameStart,
    GameFinish,
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
