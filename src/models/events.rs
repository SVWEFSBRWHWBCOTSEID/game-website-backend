use serde::{Deserialize, Serialize};

use super::{general::{GameStatus, TimeControl, Player, GameType, WinType, DrawOffer, GameKey, FriendRequest}, res::LobbyResponse};


pub enum Event {
    UserEvent(UserEvent),
    GameEvent(GameEvent),
    LobbyEvent(LobbyEvent),
}

impl Event {
    pub fn to_string(&self) -> String {
        match self {
            Event::UserEvent(e) => e.to_string(),
            Event::GameEvent(e) => e.to_string(),
            Event::LobbyEvent(e) => e.to_string(),
        }
    }
}

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

pub enum UserEvent {
    GameStartEvent(GameStartEvent),
    FriendEvent(FriendEvent),
}

impl UserEvent {
    pub fn to_string(&self) -> String {
        match self {
            UserEvent::GameStartEvent(e) => serde_json::to_string(e).unwrap(),
            UserEvent::FriendEvent(e) => serde_json::to_string(e).unwrap(),
        }
    }
}

pub enum LobbyEvent {
    NewLobbyEvent(NewLobbyEvent),
}

impl LobbyEvent {
    pub fn to_string(&self) -> String {
        match self {
            LobbyEvent::NewLobbyEvent(e) => serde_json::to_string(e).unwrap(),
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
    pub win_type: Option<WinType>,
    pub draw_offer: DrawOffer,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameFullEvent {
    pub r#type: GameEventType,
    pub rated: bool,
    pub game: GameType,
    pub time_control: TimeControl,
    pub created_at: String,
    pub first: Player,
    pub second: Player,
    pub chat: Vec<ChatMessage>,
    pub state: GameState,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub ftime: Option<i32>,
    pub stime: Option<i32>,
    pub moves: Vec<String>,
    pub status: GameStatus,
    pub win_type: Option<WinType>,
    pub draw_offer: DrawOffer,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub username: String,
    pub text: String,
    pub visibility: Visibility,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStartEvent {
    pub r#type: UserEventType,
    pub game: GameKey,
    pub id: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendEvent {
    pub r#type: UserEventType,
    pub username: String,
    pub value: FriendRequest,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewLobbyEvent {
    pub r#type: LobbyEventType,
    pub lobbies: Vec<LobbyResponse>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserEventType {
    GameStart,
    Friend,
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
pub enum LobbyEventType {
    NewLobby,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Visibility {
    Player,
    Spectator,
    Team1,
    Team2,
}
