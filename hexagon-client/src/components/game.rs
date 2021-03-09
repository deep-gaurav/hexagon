use hexagon_shared::{board::Board, colors::colors::Color, structures::{Lobby, Move, Player, PlayerMessage, PlayerStatus, SocketMessage, State}};
use yew::prelude::*;

use crate::components::home::Home;
use crate::components::peer::PeerWidget;
use crate::components::hex_board::HexBoard;

use crate::agent::notification_agent::*;
use crate::agent::socket_agent::*;

pub struct Game {
    _socket_agent: Box<dyn yew::Bridge<SocketAgent>>,
    notif_agent: Box<dyn yew::Bridge<NotificationAgent>>,
    last_move: Option<Move>,
    lobby: Lobby,
    selfid: String,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Ignore,
    PlayerJoin(Player),
    PlayerDisconnect(Player),
    LeaderChange(State),

    PlayerMove(Move),
    BoardUpdate(Board, Option<Move>),
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub lobby: Lobby,
    pub selfid: String,
}

impl Component for Game {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let agent = SocketAgent::bridge(_link.callback(|data| match data {
            AgentOutput::SocketMessage(msg) => match msg {
                SocketMessage::PlayerJoined(p, _) => Msg::PlayerJoin(p),
                SocketMessage::PlayerDisconnected(p) => Msg::PlayerDisconnect(p),
                SocketMessage::LeaderChange(leader) => Msg::LeaderChange(leader),
                SocketMessage::Moved(board, mov) => Msg::BoardUpdate(board, Some(mov)),
                _ => Msg::Ignore,
            },
            _ => Msg::Ignore,
        }));
        let notif_agent = NotificationAgent::bridge(_link.callback(|_| Msg::Ignore));
        Self {
            _socket_agent: agent,
            notif_agent,
            lobby: _props.lobby,
            link: _link,
            selfid: _props.selfid,
            last_move: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        match _msg {
            Msg::Ignore => false,
            Msg::LeaderChange(leader) => {
                self.lobby.state = leader;
                true
            }
            Msg::PlayerJoin(p) => {
                self.lobby.players.insert(p.id.clone(), p);
                true
            }
            Msg::PlayerDisconnect(p) => {
                self.lobby.players.remove(&p.id);
                true
            }
            Msg::BoardUpdate(board, mov) => {
                self.lobby.state = State::Game(board);
                self.last_move = mov;
                true
            }
            Msg::PlayerMove(mov) => {
                self._socket_agent
                    .send(AgentInput::Send(PlayerMessage::Move(mov)));
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let state = self.lobby.state.clone();
        let color = {
            let selfp = &self.lobby.players[&self.selfid];
            if let PlayerStatus::JoinedLobby(_, color) = &selfp.status {
                color.clone()
            } else {
                Color::Blue
            }
        };
        match &state {
            State::Lobby(_) => {
                html! {}
            }
            State::Game(board) => {
                html! {
                    <div class="container" style="overflow:hidden;">
                        <div class="container">
                            <h1 class="title has-text-centered">
                                {format!("Room {}",self.lobby.id)}
                            </h1>
                        </div>
                    <div class="columns center-div  is-mobile mt-3">
                    {
                        for self.lobby.players.iter().map(|p|html!{
                            <div class="column mh-2">
                            <PeerWidget key=format!("{:#?}",p) state=state.clone() peer=p.1.clone() turn={Some(board.turn.clone())}/>
                            </div>
                        })
                    }
                    </div>

                    <div class="columns">
                        <div class="column  is-three-quarters-widescreen">
                                <HexBoard  key={format!("{:?}",board)} color=color board=board move_callback=self.link.callback(|mv|Msg::PlayerMove(mv)) />
                        </div>

                    </div>
                    </div>
                }
            }
        }
    }
}
