use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, Display, EnumIter};


#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchPlayer {
    pub username: String,
    pub provisional: bool,
    pub rating: f64,
    pub rating_min: i32,
    pub rating_max: i32,
    pub first: bool,
    pub random: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub username: String,
    pub provisional: bool,
    pub rating: i32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    pub clock: ClockPreferences,
    pub game: GamePreferences,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClockPreferences {
    pub show_tenth_seconds: TenthSeconds,
    pub show_progress_bars: bool,
    pub play_critical_sound: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GamePreferences {
    pub confirm_resign: bool,
    pub board_scroll: bool,
}

#[derive(Deserialize, Serialize, Display, EnumString)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TenthSeconds {
    Always,
    Critical,
    Never,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameType {
    pub key: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeControl {
    pub initial: Option<i32>,
    pub increment: Option<i32>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Perfs {
    pub ttt: GamePerf,
    pub uttt: GamePerf,
    pub c4: GamePerf,
    pub pc: GamePerf,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GamePerf {
    pub games: i32,
    pub rating: f64,
    pub rd: f64,
    pub volatility: f64,
    pub tau: f64,
    pub prog: f64,
    pub prov: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub country: Country,
    pub location: String,
    pub bio: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Serialize, EnumIter)]
pub enum GameKey {
    TTT,
    UTTT,
    C4,
    PC,
}

#[derive(Deserialize, Serialize, Display, EnumString)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Country {
    Empty,
    Ad, Ae, Af, Ag, Ai, Al, Am, An, Ao, Aq, Ar, As, At, Au, Aw, Ax, Az,
    Ba, Bb, Bd, Be, Bf, Bg, Bh, Bi, Bj, Bl, Bm, Bn, Bo, Bq, Br, Bs, Bt, Bv, Bw, By, Bz,
    Ca, Cc, Cd, Cf, Cg, Ch, Ci, Cl, Cm, Cn, Co, Cr, Cu, Cv, Cw, Cx, Cy, Cz,
    De, Dj, Dk, Dm, Do, Dz,
    Ec, Ee, Eg, Er, Es, Et,
    Fi, Fj, Fk, Fo, Fr,
    Ga, Gb, Gd, Ge, Gf, Gg, Gh, Gi, Gl, Gm, Gn, Gq, Gr, Gs, Gt, Gu, Gw, Gy,
    Hk, Hm, Hn, Hr, Ht, Hu,
    Ic, Id, Ie, Il, Im, In, Io, Iq, Ir, Is, It,
    Us,
    Mn,
}

#[derive(Deserialize, Serialize, Display, EnumString, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FriendRequest {
    Pending,
    Accepted,
    Removed,
}

#[derive(Deserialize, Serialize, Display, EnumString, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GameStatus {
    Waiting,
    Started,
    FirstWon,
    SecondWon,
    Draw,
}

#[derive(Deserialize, Serialize, Display, EnumString, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EndType {
    Normal,
    Resign,
    Timeout,
    Disconnect,
    Stalemate,
}

#[derive(Deserialize, Serialize, Display, EnumString, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Offer {
    None,
    First,
    Second,
    Agreed,
}

#[derive(Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MoveOutcome {
    None,
    FirstWin,
    SecondWin,
    Draw,
    Stalemate,
}

#[derive(Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    First,
    Second,
    Random,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileGame {
    pub id: String,
    pub rated: bool,
    pub game: GameType,
    pub time_control: TimeControl,
    pub created_at: String,
    pub first: Player,
    pub second: Player,
    pub status: GameStatus,
    pub end_type: EndType,
}
