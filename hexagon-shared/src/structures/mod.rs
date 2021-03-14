use log::{error, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use std::collections::HashSet;
use strum::IntoEnumIterator;

use crate::board::Board;
use crate::{board::Point, colors::colors::Color};

static DEFAULT_DRAW_TIME: u32 = 90;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameType {
    TwoPlayer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TeamMode {
    Solo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub from: Point,
    pub to: Point,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lobby {
    pub id: String,
    pub players: HashMap<String, Player>,
    pub state: State,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub status: PlayerStatus,
}

impl Lobby {
    pub fn get_available_color(&self) -> Option<Color> {
        for color in Color::iter() {
            let mut taken = false;
            for p in self.players.values() {
                if let PlayerStatus::JoinedLobby(_, c) = &p.status {
                    if *c == color {
                        taken = true;
                        break;
                    }
                }
            }
            if !taken {
                return Some(color.clone());
            }
        }
        None
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum State {
    Lobby(String),
    Game(Board),
}

impl State {
    pub fn leader(&self) -> String {
        match self {
            State::Lobby(id) => id.clone(),
            State::Game(_) => {
                //FIXME: do this
                "".into()
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerStatus {
    Initiated,
    JoinedLobby(String, Color),
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum CloseCodes {
    WrongInit,
    CantCreateLobby,
    CantJoinLobbyDoestExist,
    NewSessionOpened,
    LobbyFull,
}
impl std::fmt::Display for CloseCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl CloseCodes {
    pub fn to_code(&self) -> u16 {
        match self {
            CloseCodes::WrongInit => 1003,
            CloseCodes::CantCreateLobby => 1013,
            CloseCodes::CantJoinLobbyDoestExist => 4001,
            CloseCodes::NewSessionOpened => 4002,
            CloseCodes::LobbyFull => 4003,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlayerMessage {
    Initialize(String, String),
    JoinLobby(String),
    CreateLobby,
    Ping,
    Move(Move),

    StartGame(GameType, TeamMode),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SocketMessage {
    LobbyJoined(Lobby, Color),
    PlayerJoined(Player, Color),
    PlayerDisconnected(Player),
    Close(CloseCodes),

    Moved(Board, Move),

    LeaderChange(State),
    GameStart(State),

    Pong,
}
