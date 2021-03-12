use hexagon_shared::{board::Board, colors::colors::Color, structures::{Lobby, Player, PlayerMessage, PlayerStatus, SocketMessage, State}};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::agent::notification_agent::*;
use crate::agent::socket_agent::*;

use crate::components::game::Game;
use crate::components::home::Home;
use crate::components::notification_widget::NotificationWidget;
use crate::components::room::Room;
use crate::components::backdrop::HoneyCombBackdrop;


pub struct App {
    _agent: Box<dyn yew::Bridge<SocketAgent>>,
    notif_agent: Box<dyn yew::Bridge<NotificationAgent>>,
    lobby: Option<(Lobby, Color)>,
    selfid: String,
    link: ComponentLink<Self>,
    ping_interval: yew::services::interval::IntervalTask,
}

pub enum Msg {
    Ignore,
    Ping,
    LobbyJoined(String, Lobby, Color),
    GameStart(Lobby),

    Disconnected(Option<(u16, String)>),
    PlayerDisconnected(Player),
    PlayerJoined(Player),
}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/{roomid}"]
    Room(String),
    #[to = "/"]
    Home,
}
pub fn go_to_route(route: Route) {
    use yew_router::agent::RouteRequest;
    let mut dispatcher = RouteAgentDispatcher::<()>::new();
    dispatcher.send(RouteRequest::ChangeRoute(route));
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let mut notif_agent = NotificationAgent::bridge(_link.callback(|_| Msg::Ignore));
        let agent = SocketAgent::bridge(_link.callback(|data| match data {
            AgentOutput::SocketMessage(msg) => match msg {
                SocketMessage::PlayerJoined(p, _) => Msg::PlayerJoined(p),
                SocketMessage::PlayerDisconnected(p) => Msg::PlayerDisconnected(p),
                _ => Msg::Ignore,
            },
            AgentOutput::SocketDisconnected(reason) => Msg::Disconnected(reason),
            _ => Msg::Ignore,
        }));
        let pinginterval = yew::services::IntervalService::spawn(
            std::time::Duration::from_secs(1),
            _link.callback(|_| Msg::Ping),
        );
        App {
            _agent: agent,
            notif_agent,
            lobby: None,
            link: _link,
            selfid: unsafe { crate::components::home::get_uid() },
            ping_interval: pinginterval,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        
        match _msg {
            Msg::Ping => {
                if self.lobby.is_some() {
                    self._agent.send(AgentInput::Send(PlayerMessage::Ping))
                }
                false
            }
            Msg::Ignore => false,
            Msg::LobbyJoined(selfid, lob, color) => {
                self.selfid = selfid;
                self.lobby = Some((lob, color));
                true
            }
            Msg::GameStart(lob) => {
                if let Some((_, color)) = &self.lobby {
                    self.lobby = Some((lob, color.clone()));
                }
                true
            }

            Msg::Disconnected(reason) => {
                self.notif_agent
                    .send(NotificationAgentInput::Notify(Notification {
                        notification_type: NotificationType::Error,
                        content: {
                            if let Some(reason) = reason {
                                format!(
                                    "Disconnected from server code: {}, reason: {}",
                                    reason.0, reason.1
                                )
                            } else {
                                "Disconnected from server".to_string()
                            }
                        },
                    }));
                false
            }
            Msg::PlayerJoined(p) => {
                self.notif_agent
                    .send(NotificationAgentInput::Notify(Notification {
                        notification_type: NotificationType::Info,
                        content: format!("{} joined", p.name),
                    }));
                false
            }
            Msg::PlayerDisconnected(p) => {
                self.notif_agent
                    .send(NotificationAgentInput::Notify(Notification {
                        notification_type: NotificationType::Warning,
                        content: format!("{} left", p.name),
                    }));
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let home = html! {
            <Home prefillroomid="".to_string() lobbyjoinedcb=self.link.callback(move |f:(String,Lobby,Color)|Msg::LobbyJoined(f.0,f.1,f.2))/>
        };
        
        let lobby = self.lobby.clone();
        let selfid = self.selfid.clone();
        let linkclone = self.link.clone();
        
        html! {
            <div>
                <HoneyCombBackdrop />
                <Router<AppRoute, ()>
                    render = Router::render(move |switch: AppRoute| {
                        let home = home.clone();
                        let lobby = lobby.clone();
                        let selfid = selfid.clone();
                        let link = linkclone.clone();
                        match switch {
                            AppRoute::Home=>home.clone(),
                            AppRoute::Room(_roomid)=>{
                                if let Some((lobby,color))=lobby.clone(){
                                    match &lobby.state{
                                        State::Lobby(leader)=>{
                                            html!{
                                                <Room gamestartcb=link.callback(|lob|Msg::GameStart(lob)) selfid=selfid lobby=lobby />
                                            }
                                        }
                                        State::Game(_)=>{
                                            html!{
                                                <Game selfid=selfid lobby=lobby />
                                            }
                                        }
                                    }
                                }else{
                                    html!{
                                        <Home prefillroomid=_roomid lobbyjoinedcb=linkclone.callback(move |f:(String,Lobby,Color)|Msg::LobbyJoined(f.0,f.1,f.2))/>
                                    }
                                }
                            },
                        }
                    })
                />
                <NotificationWidget/>
            </div>
        }
    }
}
