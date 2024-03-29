pub mod structures;
use hexagon_shared::{
    colors::colors::Color,
    structures::{CloseCodes, Lobby, PlayerMessage, PlayerStatus, SocketMessage, State},
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::filters::ws::{Message, WebSocket, Ws};
use warp::Filter;

use futures_util::future::FutureExt;
use futures_util::stream::StreamExt;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::RwLock;

use serde::{Deserialize, Serialize};

use log::{debug, error, info, log, trace, warn};
use pretty_env_logger;
use structures::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let wsf = warp::ws();
    let context = Context::default();
    let with_context = warp::any().map(move || context.clone());

    let logg = warp::log("WARP");

    let wshandle = wsf
        .and(with_context)
        .map(|ws: Ws, context| ws.on_upgrade(move |socket| user_connected(socket, context)))
        .with(logg);

    warp::serve(wshandle)
        .run((
            [0, 0, 0, 0],
            std::env::var("PORT")
                .unwrap_or("3012".to_owned())
                .parse()
                .unwrap(),
        ))
        .await;
}

async fn user_connected(websocket: WebSocket, context: Context) {
    info!("Websocket Connection Received");
    println!("Websocket Connection Received");
    let (ws_tx, mut ws_rx) = websocket.split();

    let (tx, rx) = unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);
    tokio::task::spawn(rx.forward(ws_tx).map(|result| {
        if let Err(e) = result {
            warn!("websocket send error {:#?}", e);
        }
    }));

    let mut player: Option<ServerPlayer> = None;

    // TODO: Add timer for closing
    // let timeoutfuture = tokio::time::delay_for(std::time::Duration::from_secs(3));

    if let Some(result) = ws_rx.next().await {
        match result {
            Ok(msg) => {
                if let Ok(msg)= msg.to_str() {
                    match serde_json::from_str(msg) {
                        Ok(message) => match message {
                            PlayerMessage::Initialize(id, name) => {
                                info!("Intialize player id {:#} name {:#?}", id, name);
                                player = Some(ServerPlayer {
                                    id,
                                    name,
                                    send_channel: tx.clone(),
                                    status: PlayerStatus::Initiated,
                                });
                            }
                            _ => {
                                warn!(
                                    "First message not initialize, closing connection {:#?}",
                                    msg
                                );
                                if let Err(e) = tx.send(Ok(Message::close_with(
                                    CloseCodes::WrongInit as u8,
                                    CloseCodes::WrongInit.to_string(),
                                ))) {
                                    error!("Cant close connection {:#?}", e);
                                }
                            }
                        },
                        Err(err) => {
                            debug!("Received message is incorrect format, {:#}", err);
                            if let Err(e) = tx.send(Ok(Message::close_with(
                                CloseCodes::WrongInit as u8,
                                CloseCodes::WrongInit.to_string(),
                            ))) {
                                error!("Cant close connection {:#?}", e);
                            }
                        }
                    }
                } else {
                    error!("Binary not supported {:#?}", msg);
                    if let Err(e) = tx.send(Ok(Message::close_with(
                        CloseCodes::WrongInit as u8,
                        CloseCodes::WrongInit.to_string(),
                    ))) {
                        error!("Cant close connection {:#?}", e);
                    }
                }
            }
            Err(e) => {
                warn!("Websocket error {:#?} player : {:#?}", e, player);
            }
        };
    }

    match &mut player {
        Some(player) => {
            if let Some(result) = ws_rx.next().await {
                match result {
                    Ok(msg) => {
                        if let Ok(msg)=msg.to_str() {
                            match serde_json::from_str(msg) {
                                Ok(message) => match message {
                                    PlayerMessage::CreateLobby => {
                                        use rand::{distributions::Alphanumeric, Rng};
                                        let lobbyid: String = {
                                            rand::thread_rng()
                                                .sample_iter(&Alphanumeric)
                                                .take(5)
                                                .map(char::from)
                                                .collect()
                                        };
                                        let privatelobbies =
                                            &mut context.write().await.private_lobbies;
                                        if let Some(_lob) = privatelobbies.get(&lobbyid) {
                                            error!(
                                                "Lobby exist with id {:#?} {:#?}, returning error",
                                                lobbyid, _lob
                                            );
                                            player.close(CloseCodes::CantCreateLobby);
                                        } else {
                                            player.status = PlayerStatus::JoinedLobby(
                                                lobbyid.clone(),
                                                Color::Red,
                                            );
                                            let lobby = ServerLobby::new_with_player(
                                                lobbyid.clone(),
                                                player.clone(),
                                            );
                                            privatelobbies.insert(lobbyid, lobby.clone());

                                            info!("Player {:#?} joined lobby {:#?}", player, lobby);
                                            player.send(SocketMessage::LobbyJoined(
                                                lobby.into(),
                                                Color::Red,
                                            ));
                                        }
                                    }
                                    PlayerMessage::JoinLobby(lobbyid) => {
                                        let privatelobbies =
                                            &mut context.write().await.private_lobbies;
                                        if let Some(lobby) = privatelobbies.get_mut(&lobbyid) {
                                            if let Some(color) =
                                                Lobby::from(lobby.clone()).get_available_color()
                                            {
                                                player.status = PlayerStatus::JoinedLobby(
                                                    lobby.id.clone(),
                                                    color.clone(),
                                                );
                                                lobby.add_player(player.clone());
                                                info!(
                                                    "Player {:#?} joined lobby {:#?}",
                                                    player, lobby
                                                );
                                                player.send(SocketMessage::LobbyJoined(
                                                    lobby.clone().into(),
                                                    color,
                                                ));
                                            } else {
                                                player.close(CloseCodes::LobbyFull)
                                            }
                                        } else {
                                            player.close(CloseCodes::CantJoinLobbyDoestExist)
                                        }
                                    }
                                    _ => {}
                                },
                                Err(e) => {
                                    log::warn!("Message is not player message {:#?}", e);
                                }
                            }
                        } else {
                            error!("Not Text message {:#?}", msg)
                        }
                    }
                    Err(e) => {
                        log::warn!("Websocket error {:#?}", e);
                    }
                }
            }
        }
        None => {
            warn!("Player not initialized");
            if let Err(e) = tx.send(Ok(Message::close_with(
                CloseCodes::WrongInit as u8,
                CloseCodes::WrongInit.to_string(),
            ))) {
                error!("Cant close connection {:#?}", e);
            }
        }
    }

    if let Some(player) = player {
        if let PlayerStatus::JoinedLobby(lobbyid, _) = player.status {
            let messageblock = websocket_msg(&player.id, &lobbyid, &context, ws_rx);

            messageblock.await;
        }
    }
}

async fn player_message(player_id: &str, lobbyid: &str, context: &Context, message: PlayerMessage) {
    let lobbies = &mut context.write().await.private_lobbies;
    if let Some(lobby) = lobbies.get_mut(lobbyid) {
        let colors = lobby
            .players
            .iter()
            .map(|(id, p)| {
                if let PlayerStatus::JoinedLobby(_, c) = p.status {
                    Some(c)
                } else {
                    None
                }
            })
            .filter_map(|f| f)
            .collect::<Vec<_>>();
        if let Some(player) = lobby.players.get_mut(player_id) {
            match message {
                PlayerMessage::Ping => {
                    player.send(SocketMessage::Pong);
                }
                PlayerMessage::StartGame(game_type, team_mode) => {
                    let pid = &player.id.clone();
                    lobby.start_game(pid, game_type, team_mode);
                }
                PlayerMessage::Move(mov) => {
                    if let PlayerStatus::JoinedLobby(_, color) = &player.status {
                        if let State::Game(board) = &mut lobby.state {
                            if &board.turn == color {
                                if board.is_move_legal(&mov) {
                                    board.apply_move(&mov);
                                    let next_color = colors
                                        .into_iter()
                                        .find(|c| c != &board.turn)
                                        .unwrap_or(board.turn);
                                    board.change_turn(next_color);
                                    let newboard = board.clone();
                                    lobby.broadcast(SocketMessage::Moved(newboard, mov.clone()))
                                }
                            }
                        }
                    }
                }
                msg => {
                    warn!("Received Unexpected Player message {:#?}", msg);
                }
            }
        } else {
            error!("Player id {:#?} not found in Lobby {:#?}", player_id, lobby);
        }
    } else {
        error!(
            "Lobby id {:#?} not found for player id {:#?}",
            lobbyid, player_id
        );
    }
}

async fn player_disconnect(player_id: &str, lobbyid: &str, context: &Context) {
    log::debug!("Player Disconnected {:#?}", player_id);
    let lobbies = &mut context.write().await.private_lobbies;
    if let Some(lobby) = lobbies.get_mut(lobbyid) {
        lobby.remove_player(player_id);
        if lobby.players.is_empty() {
            let lobid = lobby.id.clone();
            lobbies.remove(&lobid);
        }
    }
}

async fn websocket_msg(
    player_id: &str,
    lobbyid: &str,
    context: &Context,
    mut ws_rx: futures_util::stream::SplitStream<WebSocket>,
) {
    while let Some(msg) = ws_rx.next().await {
        match msg {
            Ok(message) => {
                if message.is_close() {
                    // player_disconnect(&player_id, &lobbyid, &context);
                    break;
                } else if let Ok(msg)=message.to_str() {
                    match serde_json::from_str(msg) {
                        Ok(player_msg) => {
                            player_message(&player_id, &lobbyid, &context, player_msg).await;
                        }
                        Err(er) => {
                            warn!("Received message not Player Message {:#?}", er);
                        }
                    }
                } else {
                    warn!("Received message not text {:#?}", message);
                }
            }
            Err(er) => {
                warn!("websocket error {:#}", er);
            }
        }
    }

    player_disconnect(&player_id, &lobbyid, &context).await;
}
