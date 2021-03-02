use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use log::{error, warn};

use std::collections::HashSet;
use strum::IntoEnumIterator;

use crate::colors::colors::Color;
use crate::board::Board;

static DEFAULT_DRAW_TIME: u32 = 90;

#[derive(Default)]
pub struct Lobbies {
    pub private_lobbies: HashMap<String, Lobby>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Lobby {
    pub id: String,
    pub players: HashMap<String, Player>,
    pub state: State,
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

#[derive(Debug, Serialize, Clone)]
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

impl Lobby {
    pub fn new_with_player(id: String, player: Player) -> Self {
        let mut map = HashMap::new();
        map.insert(player.id.clone(), player.clone());
        Lobby {
            id,
            players: map,
            state: State::Lobby(player.id.clone()),
        }
    }

    pub fn add_player(&mut self, player: Player) -> Self {
        if let PlayerStatus::JoinedLobby(_, color) = &player.status {
            self.broadcast(SocketMessage::PlayerJoined(player.clone(), color.clone()));
        }
        if let Some(oldplayer) = self.players.insert(player.id.clone(), player.clone()) {
            log::warn!("Old player {:#?} replaced by {:#?}", oldplayer, player);
            oldplayer.close(CloseCodes::NewSessionOpened);
        }
        self.clone()
    }

    pub fn broadcast(&self, message: SocketMessage) {
        for p in self.players.iter() {
            p.1.send(message.clone());
        }
    }

    pub fn broadcast_except(&self, id: &str, message: SocketMessage) {
        for p in self.players.iter() {
            if p.0 != id {
                p.1.send(message.clone());
            }
        }
    }

    pub fn assignnewleader(&mut self) {
        use itertools::Itertools;
        let mut players = self.players.keys().sorted();
        while let Some(pid) = players.next() {
            if pid == &self.state.leader() {
                if let Some(pid) = players.next() {
                    self.state = {
                        match &self.state {
                            State::Game(board) => State::Game(board.clone()),
                            State::Lobby(_) => State::Lobby(pid.clone()),
                        }
                    };
                    self.broadcast(SocketMessage::LeaderChange(self.state.clone()));
                    return;
                }
            }
        }
        if let Some(pid) = self.players.keys().sorted().next() {
            self.state = match &self.state {
                State::Game(board) => State::Game(board.clone()),
                State::Lobby(_) => State::Lobby(pid.clone()),
            };
            self.broadcast(SocketMessage::LeaderChange(self.state.clone()));
        }
    }

    pub fn remove_player(&mut self, playerid: &str) {
        if playerid == self.state.leader() {
            self.assignnewleader();
        }
        if let Some(player) = self.players.remove(playerid) {
            log::debug!("Player removed {:#?}", player);
            self.broadcast(SocketMessage::PlayerDisconnected(player));
        }
    }

    pub fn start_game(&mut self, playerid: &str, game_type: GameType, team_mode: TeamMode) {
        match &self.state {
            State::Lobby(pid) => {
                if playerid == pid {
                    self.state = State::Game({
                        match game_type {
                            GameType::TwoPlayer => Board::init_2p(),
                            GameType::FourPlayer | GameType::ThreePlayer => {
                                Board::init_4p(team_mode, game_type)
                            }
                        }
                    });
                    self.broadcast(SocketMessage::GameStart(self.state.clone()));
                } else {
                    warn!("Only leader {:#} can start game", pid);
                }
            }
            State::Game(_) => {
                warn!("Cant start game, already in game state");
            }
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    #[serde(skip)]
    pub send_channel: UnboundedSender<Result<Message, warp::Error>>,
    pub status: PlayerStatus,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerStatus {
    Initiated,
    JoinedLobby(String, Color),
}

#[derive(Debug, Serialize, Copy, Clone)]
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
    fn to_code(&self) -> u16 {
        match self {
            CloseCodes::WrongInit => 1003,
            CloseCodes::CantCreateLobby => 1013,
            CloseCodes::CantJoinLobbyDoestExist => 4001,
            CloseCodes::NewSessionOpened => 4002,
            CloseCodes::LobbyFull => 4003,
        }
    }
}

pub type Context = Arc<RwLock<Lobbies>>;

#[derive(Debug, Serialize, Deserialize)]
pub enum PlayerMessage {
    Initialize(String, String),
    JoinLobby(String),
    CreateLobby,
    Ping,
    Move(Move),

    StartGame(GameType, TeamMode),
}

#[derive(Debug, Serialize, Clone)]
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
