use hexagon_shared::{colors::colors::Color, structures::{Lobby, PlayerMessage, SocketMessage}};
use yew::prelude::*;

use crate::agent::socket_agent::{AgentInput, AgentOutput, SocketAgent};
use crate::components::avatar::avatar;
use lazy_static::lazy_static;

use wasm_bindgen::*;

lazy_static! {
    static ref SIGNAL_URL: String = String::from(env!("SERVER_URL"));
}

pub struct Home {
    name: String,
    room_id: String,
    link: ComponentLink<Self>,
    is_connecting: bool,
    socket_agent: Box<dyn yew::Bridge<SocketAgent>>,
    props: Props,
}

#[derive(Debug, Properties, Clone)]
pub struct Props {
    pub lobbyjoinedcb: Callback<(String, Lobby, Color)>,
    pub prefillroomid: String,
}

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    pub fn get_uid() -> String;
}

pub enum Msg {
    Connected,
    Disconnected(Option<(u16, String)>),
    ErrorConnecting,
    Connect,
    Ignore,
    LobbyJoined(Lobby, Color),
    NameChange(String),
    RoomIdChange(String),
}

impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let agent = SocketAgent::bridge(_link.callback(|data| match data {
            AgentOutput::SocketConnected => Msg::Connected,

            AgentOutput::SocketMessage(msg) => match msg {
                SocketMessage::LobbyJoined(lobby, color) => Msg::LobbyJoined(lobby, color),
                SocketMessage::Close(_) => Msg::Disconnected(None),
                _ => Msg::Ignore,
            },
            AgentOutput::SocketDisconnected(reason) => Msg::Disconnected(reason),
            AgentOutput::SocketErrorConnecting => Msg::ErrorConnecting,
        }));
        Home {
            name: "".to_string(),
            room_id: _props.prefillroomid.clone(),
            link: _link,
            socket_agent: agent,
            is_connecting: false,
            props: _props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        match _msg {
            Msg::NameChange(name) => {
                self.name = name;
                true
            }
            Msg::RoomIdChange(id) => {
                self.room_id = id;
                true
            }
            Msg::Connect => {
                if self.name.is_empty() {
                    false
                } else {
                    self.is_connecting = true;
                    self.socket_agent
                        .send(AgentInput::Connect(SIGNAL_URL.to_string()));
                    true
                }
            }
            Msg::Connected => {
                let uid = unsafe { get_uid() };
                log::info!("uid is {:#?}", uid);
                self.socket_agent
                    .send(AgentInput::Send(PlayerMessage::Initialize(
                        uid,
                        self.name.to_string(),
                    )));
                if self.room_id.is_empty() {
                    self.socket_agent
                        .send(AgentInput::Send(PlayerMessage::CreateLobby));
                } else {
                    self.socket_agent
                        .send(AgentInput::Send(PlayerMessage::JoinLobby(
                            self.room_id.clone(),
                        )));
                }
                false
            }
            Msg::Disconnected(_) => {
                self.is_connecting = false;
                true
            }
            Msg::ErrorConnecting => {
                self.is_connecting = false;
                true
            }
            Msg::LobbyJoined(lob, color) => {
                crate::app::go_to_route(yew_router::route::Route::from(
                    crate::app::AppRoute::Room(lob.id.clone()),
                ));
                let uid = unsafe { get_uid() };
                self.props.lobbyjoinedcb.emit((uid, lob, color));
                true
            }
            Msg::Ignore => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <section class="section">
                <div class="container">
                    <h1 class="title has-text-centered">
                        {"Hexagon"}
                    </h1>
                </div>

            </section>
            <section class="section has-text-centered">
                <div class="container" style="display:inline-flex;">
                <div class="box">
                    {
                        avatar(&self.name, &Color::Blue)
                    }

                    <div class="container mt-2">
                        <fieldset disabled=self.is_connecting>
                        <div class="field">
                            <div class="control">
                                <input oninput=self.link.callback(|msg:InputData|Msg::NameChange(msg.value)) class="input" type="text" placeholder="Enter Name"/>
                            </div>
                        </div>
                        </fieldset>
                    </div>
                    <div class="container mt-2">
                        <fieldset disabled=self.name.is_empty() || self.is_connecting>
                        <div class="field has-addons">
                            <div class="control ">
                                <input value=self.room_id.clone() oninput=self.link.callback(|msg:InputData|Msg::RoomIdChange(msg.value)) class="input" type="text" placeholder="Enter Room Id to join"/>
                            </div>
                        </div>
                        </fieldset>
                    </div>
                    <div class="container mt-2">
                        <button class="control" disabled=self.is_connecting>
                            <a key=self.is_connecting.to_string() onclick=self.link.callback(|_|Msg::Connect) class=format!("button is-outlined is-primary {}",if self.is_connecting{"is-loading"}else{""})>
                                {
                                    if(self.room_id.is_empty()){
                                        "Create"
                                    }else{
                                        "Join"
                                    }
                                }
                            </a>
                        </button>
                    </div>
                </div>
                </div>
            </section>
            </>
        }
    }
}
