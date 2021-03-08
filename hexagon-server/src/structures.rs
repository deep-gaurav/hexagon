use std::{collections::HashMap, sync::Arc};

use itertools::Itertools;
use log::{debug, error, info, warn};

use hexagon_shared::{
    board::Board,
    colors::colors::Color,
    structures::{
        CloseCodes, GameType, Lobby, Player, PlayerStatus, SocketMessage, State, TeamMode,
    },
};
use tokio::sync::{mpsc::UnboundedSender, RwLock};

use serde::{Deserialize, Serialize};
use warp::ws::Message;
#[derive(Default)]
pub struct Lobbies {
    pub private_lobbies: HashMap<String, ServerLobby>,
}

pub type Context = Arc<RwLock<Lobbies>>;
#[derive(Debug, Clone)]
pub struct ServerPlayer {
    pub id: String,
    pub name: String,
    pub send_channel: UnboundedSender<Result<Message, warp::Error>>,
    pub status: PlayerStatus,
}

impl From<ServerPlayer> for Player {
    fn from(serverplayer: ServerPlayer) -> Self {
        Self {
            id: serverplayer.id,
            name: serverplayer.name,
            status: serverplayer.status,
        }
    }
}

impl ServerPlayer {
    pub fn send(&self, message: SocketMessage) {
        match bincode::serialize(&message) {
            Ok(bytes) => {
                if let Err(er) = self.send_channel.send(Ok(Message::binary(bytes))) {
                    log::warn!("Cant send message to player {:#?} error {:#?}", self, er);
                }
            }
            Err(err) => {
                log::error!("Cant serialize {:#?} error {:#?}", message, err);
            }
        }
    }
    pub fn close(&self, code: CloseCodes) {
        warn!("Closing connection to {:#?} code: {:#?}", &self, code);
        if let Err(er) = self
            .send_channel
            .send(Ok(Message::close_with(code.to_code(), code.to_string())))
        {
            error!(
                "Cant send close message to player {:#?} error {:#?}",
                self, er
            );
        }
    }
}

#[derive(Debug, Clone)]
pub struct ServerLobby {
    pub id: String,
    pub players: HashMap<String, ServerPlayer>,
    pub state: State,
}

impl From<ServerLobby> for Lobby {
    fn from(lobby: ServerLobby) -> Self {
        Self {
            id: lobby.id,
            players: lobby
                .players
                .into_iter()
                .map(|s| (s.0, Player::from(s.1)))
                .collect(),
            state: lobby.state,
        }
    }
}

impl ServerLobby {
    pub fn new_with_player(id: String, player: ServerPlayer) -> Self {
        let mut map = HashMap::new();
        map.insert(player.id.clone(), player.clone());
        Self {
            id,
            players: map,
            state: State::Lobby(player.id.clone()),
        }
    }

    pub fn add_player(&mut self, player: ServerPlayer) -> Self {
        if let PlayerStatus::JoinedLobby(_, color) = &player.status {
            self.broadcast(SocketMessage::PlayerJoined(
                player.clone().into(),
                color.clone(),
            ));
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
            self.broadcast(SocketMessage::PlayerDisconnected(Player::from(player)));
        }
    }

    pub fn start_game(&mut self, playerid: &str, game_type: GameType, team_mode: TeamMode) {
        match &self.state {
            State::Lobby(pid) => {
                if playerid == pid {
                    if let Some(player) = self.players.get(pid) {
                        if let PlayerStatus::JoinedLobby(_, color) = player.status {
                            let othercolor = {
                                let op = self.players.values().find(|p| {
                                    if let PlayerStatus::JoinedLobby(_, ncolor) = p.status {
                                        color != ncolor
                                    } else {
                                        false
                                    }
                                });
                                if let Some(p) = op {
                                    if let PlayerStatus::JoinedLobby(_, c) = p.status {
                                        c
                                    } else {
                                        Color::DarkRed
                                    }
                                } else {
                                    Color::DarkRed
                                }
                            };

                            self.state = State::Game({
                                match game_type {
                                    GameType::TwoPlayer => {
                                        Board::generate_hexagon(8, color, othercolor)
                                    }
                                }
                            });
                            self.broadcast(SocketMessage::GameStart(self.state.clone()));
                        }
                    }
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
