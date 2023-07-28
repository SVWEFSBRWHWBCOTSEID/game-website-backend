use serde::{Deserialize, Serialize};

use super::{general::{GameStatus, TimeControl, Player, GameType, WinType, Offer, GameKey, FriendRequest}, res::LobbyResponse};


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
    ChatGameEvent(ChatGameEvent),
    GameStateEvent(GameStateEvent),
    GameFullEvent(GameFullEvent),
    RematchEvent(RematchEvent),
}

impl GameEvent {
    pub fn to_string(&self) -> String {
        match self {
            GameEvent::ChatMessageEvent(e) => serde_json::to_string(e).unwrap(),
            GameEvent::ChatGameEvent(e) => serde_json::to_string(e).unwrap(),
            GameEvent::GameStateEvent(e) => serde_json::to_string(e).unwrap(),
            GameEvent::GameFullEvent(e) => serde_json::to_string(e).unwrap(),
            GameEvent::RematchEvent(e) => serde_json::to_string(e).unwrap(),
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
    AllLobbiesEvent(AllLobbiesEvent),
}

impl LobbyEvent {
    pub fn to_string(&self) -> String {
        match self {
            LobbyEvent::AllLobbiesEvent(e) => serde_json::to_string(e).unwrap(),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Chat {
    ChatMessage {
        username: String,
        text: String,
        visibility: Visibility,
    },
    ChatGame {
        message: String,
    },
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
pub struct ChatGameEvent {
    pub r#type: GameEventType,
    pub message: String,
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
    pub draw_offer: Offer,
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
    pub chat: Vec<Chat>,
    pub state: GameState,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RematchEvent {
    pub r#type: GameEventType,
    pub rematch_offer: Offer,
    pub id: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub ftime: Option<i32>,
    pub stime: Option<i32>,
    pub moves: Vec<String>,
    pub status: GameStatus,
    pub win_type: Option<WinType>,
    pub draw_offer: Offer,
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
pub struct AllLobbiesEvent {
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
    ChatGame,
    GameState,
    GameFull,
    Rematch,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LobbyEventType {
    AllLobbies,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Visibility {
    Player,
    Spectator,
    Team1,
    Team2,
}
