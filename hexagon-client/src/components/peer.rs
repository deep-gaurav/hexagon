use hexagon_shared::{colors::colors::Color, structures::{Player, PlayerStatus, State}};
use web_sys::Blob;
use yew::prelude::*;

use crate::agent::socket_agent::*;


use wasm_bindgen::prelude::*;

pub struct PeerWidget {
    _socket_agent: Box<dyn yew::Bridge<SocketAgent>>,
    state: State,
    turn: Option<Color>,
    link: ComponentLink<Self>,

    peer: Player,
}

pub enum Msg {
    Ignore,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub peer: Player,
    pub state: State,
    pub turn: Option<Color>,
}

impl Component for PeerWidget {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let agent = SocketAgent::bridge(_link.callback(|data| match data {
            _ => Msg::Ignore,
        }));
        // agent.send(AgentInput::LobbyInput(LobbyInputs::RequestLobby));
        Self {
            _socket_agent: agent,
            link: _link,
            peer: _props.peer,
            state: _props.state,
            turn: _props.turn,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        match _msg {
            Msg::Ignore => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.state = _props.state;
        self.peer = _props.peer;
        self.turn = _props.turn;
        true
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn view(&self) -> Html {
        use crate::components::avatar::avatar;
        let color = {
            match &self.peer.status {
                PlayerStatus::Initiated => Color::Blue,
                PlayerStatus::JoinedLobby(_, color) => *color,
            }
        };
        let border_color = {
            if let Some(color) = &self.turn {
                if let PlayerStatus::JoinedLobby(_, c) = &self.peer.status {
                    if color == c {
                        "green"
                    } else {
                        "black"
                    }
                } else {
                    "black"
                }
            } else {
                "black"
            }
        };
        html! {
            <>
                <div class="container has-text-centered">

                    <div id=&self.peer.id style=format!("display:inline-block;border-width:5px;border-style:solid;border-radius:50%;border-color:{}",border_color)>
                    {
                        avatar(&self.peer.name,&color)
                    }
                    </div>
                    <div>
                    {
                        &self.peer.name
                    }
                    </div>
                </div>
            </>
        }
    }
}
