use serde::{Deserialize, Serialize};
use std::time::Duration;
use yew::agent::*;
use yew::prelude::*;
use yew::services::interval::*;

use log::{debug, info};

#[derive(Debug)]
pub struct Compo {
    handle: HandlerId,
    state: AnimState,
    progress: f32,
    duration: Duration,
}

#[derive(Debug)]
pub struct AnimAgent {
    fps: u64,
    interval_task: IntervalTask,
    link: AgentLink<Self>,
    handlers: Vec<Compo>,
}

#[derive(Debug)]
pub enum AnimState {
    Forwarding,
    BackWarding,
    Stopped,
}

pub enum Msg {
    Tick,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AgentInput {
    ChangeDuration(Duration),
    Forward,
    BackWard,
    Pause,
    Reset,
    // GetProgress,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AgentOutput {
    End(f32),
    Progress(f32),
}

impl Agent for AnimAgent {
    type Reach = Context<Self>;

    type Message = Msg;

    type Input = AgentInput;

    type Output = AgentOutput;

    fn create(link: AgentLink<Self>) -> Self {
        log::info!("start agent");
        let fps = 60;
        let task = IntervalService::spawn(
            Duration::from_millis(1000 / fps),
            link.callback(|_| Msg::Tick),
        );
        Self {
            fps,
            interval_task: task,
            link,
            handlers: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Tick => {
                for comp in self.handlers.iter_mut() {
                    match comp.state {
                        AnimState::Forwarding => {
                            if comp.progress < 1.0 {
                                comp.progress +=
                                    1.0 / ((self.fps * comp.duration.as_secs()) as f32);
                                comp.progress = comp.progress.clampi(0.0, 1.0);
                                self.link
                                    .respond(comp.handle, AgentOutput::Progress(comp.progress));
                            } else {
                                comp.state = AnimState::Stopped;
                                self.link
                                    .respond(comp.handle, AgentOutput::End(comp.progress));
                            }
                        }
                        AnimState::BackWarding => {
                            if comp.progress > 0.0 {
                                comp.progress -=
                                    1.0 / ((self.fps * comp.duration.as_secs()) as f32);
                                comp.progress = comp.progress.clampi(0.0, 1.0);
                                self.link
                                    .respond(comp.handle, AgentOutput::Progress(comp.progress));
                            } else {
                                comp.state = AnimState::Stopped;
                                self.link
                                    .respond(comp.handle, AgentOutput::End(comp.progress));
                            }
                        }
                        AnimState::Stopped => {}
                    }
                }
            }
        }
    }

    fn connected(&mut self, _id: HandlerId) {
        self.handlers.push(Compo {
            handle: _id,
            state: AnimState::Stopped,
            progress: 0.0,
            duration: Duration::from_secs(1),
        });
    }

    fn disconnected(&mut self, _id: HandlerId) {
        if let Some(pos) = self.handlers.iter().position(|p| p.handle == _id) {
            self.handlers.remove(pos);
        }
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        log::info!("Msg Received");
        match msg {
            AgentInput::ChangeDuration(duration) => {
                if let Some(pos) = self.handlers.iter().position(|f| f.handle == id) {
                    self.handlers[pos].duration = duration;
                }
            }
            AgentInput::Forward => {
                if let Some(pos) = self.handlers.iter().position(|f| f.handle == id) {
                    self.handlers[pos].state = AnimState::Forwarding;
                }
            }
            AgentInput::BackWard => {
                if let Some(pos) = self.handlers.iter().position(|f| f.handle == id) {
                    self.handlers[pos].state = AnimState::BackWarding;
                }
            }
            AgentInput::Pause => {
                if let Some(pos) = self.handlers.iter().position(|f| f.handle == id) {
                    self.handlers[pos].state = AnimState::Stopped;
                }
            }
            AgentInput::Reset => {
                if let Some(pos) = self.handlers.iter().position(|f| f.handle == id) {
                    self.handlers[pos].progress = 0.0;
                }
            }
        }
    }
}

// impl AnimAgent {
//     fn broadcast(&self, msg: &AgentOutput) {
//         for handler in self.handlers.iter() {
//             self.link.respond(*handler, msg.clone());
//
//     }
// }

impl Clamp for f32 {}

trait Clamp: PartialOrd {
    fn clampi(self, min: Self, max: Self) -> Self
    where
        Self: std::marker::Sized,
    {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}
