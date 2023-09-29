use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use strum_macros::{Display, EnumString};

use super::general::{GameStatus, TimeControl, Player, GameType, EndType, Offer, GameKey, FriendRequest, Preferences, Side};
use super::res::LobbyResponse;


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
    ChatAlertEvent(ChatAlertEvent),
    GameStateEvent(GameStateEvent),
    GameFullEvent(GameFullEvent),
    RematchEvent(RematchEvent),
}

impl GameEvent {
    pub fn to_string(&self) -> String {
        match self {
            GameEvent::ChatMessageEvent(e) => serde_json::to_string(e).unwrap(),
            GameEvent::ChatAlertEvent(e) => serde_json::to_string(e).unwrap(),
            GameEvent::GameStateEvent(e) => serde_json::to_string(e).unwrap(),
            GameEvent::GameFullEvent(e) => serde_json::to_string(e).unwrap(),
            GameEvent::RematchEvent(e) => serde_json::to_string(e).unwrap(),
        }
    }
}

pub enum UserEvent {
    GameStartEvent(GameStartEvent),
    PreferencesUpdateEvent(PreferencesUpdateEvent),
    FriendEvent(FriendEvent),
    UserMessageEvent(UserMessageEvent),
    ChallengeEvent(ChallengeEvent),
    ChallengeDeclinedEvent(ChallengeDeclinedEvent),
}

impl UserEvent {
    pub fn to_string(&self) -> String {
        match self {
            UserEvent::GameStartEvent(e) => serde_json::to_string(e).unwrap(),
            UserEvent::PreferencesUpdateEvent(e) => serde_json::to_string(e).unwrap(),
            UserEvent::FriendEvent(e) => serde_json::to_string(e).unwrap(),
            UserEvent::UserMessageEvent(e) => serde_json::to_string(e).unwrap(),
            UserEvent::ChallengeEvent(e) => serde_json::to_string(e).unwrap(),
            UserEvent::ChallengeDeclinedEvent(e) => serde_json::to_string(e).unwrap(),
        }
    }
}

pub enum LobbyEvent {
    LobbyFullEvent(LobbyFullEvent),
    AllLobbiesEvent(AllLobbiesEvent),
    PlayerStatsEvent(PlayerStatsEvent),
}

impl LobbyEvent {
    pub fn to_string(&self) -> String {
        match self {
            LobbyEvent::LobbyFullEvent(e) => serde_json::to_string(e).unwrap(),
            LobbyEvent::AllLobbiesEvent(e) => serde_json::to_string(e).unwrap(),
            LobbyEvent::PlayerStatsEvent(e) => serde_json::to_string(e).unwrap(),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Chat {
    ChatMessage {
        username: String,
        text: String,
        visibility: Visibility,
    },
    ChatAlert {
        message: String,
    },
}

impl Serialize for Chat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            Chat::ChatMessage { username, text, visibility } => {
                let mut state = serializer.serialize_struct("ChatMessage", 3)?;
                state.serialize_field("username", username)?;
                state.serialize_field("text", text)?;
                state.serialize_field("visibility", visibility)?;
                state.end()
            }
            Chat::ChatAlert { message } => {
                let mut state = serializer.serialize_struct("ChatAlert", 1)?;
                state.serialize_field("message", message)?;
                state.end()
            }
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
pub struct ChatAlertEvent {
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
    pub end_type: Option<EndType>,
    pub draw_offer: Offer,
    pub frating_diff: Option<i32>,
    pub srating_diff: Option<i32>,
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
    pub end_type: Option<EndType>,
    pub draw_offer: Offer,
    pub frating_diff: Option<i32>,
    pub srating_diff: Option<i32>,
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
pub struct PreferencesUpdateEvent {
    pub r#type: UserEventType,
    pub preferences: Preferences,
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
pub struct UserMessageEvent {
    pub r#type: UserEventType,
    pub username: String,
    pub text: String,
    pub created_at: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeEvent {
    pub r#type: UserEventType,
    pub username: String,
    pub opponent: String,
    pub id: String,
    pub rated: bool,
    pub game: GameType,
    pub time_control: TimeControl,
    pub side: Side,
    pub created_at: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeDeclinedEvent {
    pub r#type: UserEventType,
    pub opponent: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LobbyFullEvent {
    pub r#type: LobbyEventType,
    pub lobbies: Vec<LobbyResponse>,
    pub players: i32,
    pub games: i32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllLobbiesEvent {
    pub r#type: LobbyEventType,
    pub lobbies: Vec<LobbyResponse>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStatsEvent {
    pub r#type: LobbyEventType,
    pub players: i32,
    pub games: i32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserEventType {
    GameStart,
    PreferencesUpdate,
    Friend,
    UserMessage,
    Challenge,
    ChallengeDeclined,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GameEventType {
    ChatMessage,
    ChatAlert,
    GameState,
    GameFull,
    Rematch,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LobbyEventType {
    LobbyFull,
    AllLobbies,
    PlayerStats,
}

#[derive(Deserialize, Serialize, Display, EnumString)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Visibility {
    Player,
    Spectator,
    Team1,
    Team2,
}
