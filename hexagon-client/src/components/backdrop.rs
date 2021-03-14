use crate::ui::GameColors;
use crate::{
    agent::anim_agent::AgentInput, agent::anim_agent::AnimAgent, components::hex_board::HexBoard,
};
use hexagon_shared::{board::Board, colors::colors::Color, models::OffsetCoord, structures::Move};
use rand::seq::SliceRandom;
use yew::agent::*;
use yew::prelude::*;

pub struct HoneyCombBackdrop {
    pub board: Board,
    pub link: ComponentLink<Self>,
    pub wcell: i32,
    pub hcell: i32,
    pub anim_agent: Box<dyn yew::Bridge<AnimAgent>>,
}

pub enum Msg {
    Ignore,
    MakeMove,
}



#[derive(Debug, Clone, Properties)]
pub struct Props {}

impl HoneyCombBackdrop {
    fn get_next_color(&self) -> Color {
        if self.board.turn == Color::Red {
            Color::Green
        } else {
            Color::Red
        }
    }
    fn generate_board() -> (Board, f32, f32) {
        let width: f32 = yew::utils::window()
            .inner_width()
            .expect("Width not present")
            .as_f64()
            .expect("Not number") as f32;
        let height: f32 = yew::utils::window()
            .inner_height()
            .expect("height not present")
            .as_f64()
            .expect("Not number") as f32;
        let height_ratio = width / height;

        let hex_r = 1.1547005;
        let cellwidth = 100.0;
        let wcell = width / cellwidth;
        let hcell = height / cellwidth * hex_r;
        let board =
            Board::generate_honeycomb(wcell as i32, hcell as i32, 5, Color::Green, Color::Red);
        (board, hcell, wcell)
    }
}

impl Component for HoneyCombBackdrop {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let (board, hcell, wcell) = Self::generate_board();
        let mut anim_agent = AnimAgent::bridge(link.callback(|msg| match msg {
            crate::agent::anim_agent::AgentOutput::End(_) => Msg::MakeMove,
            crate::agent::anim_agent::AgentOutput::Progress(_) => Msg::Ignore,
        }));
        anim_agent.send(AgentInput::ChangeDuration(std::time::Duration::from_secs(1)));
        anim_agent.send(AgentInput::Forward);
        Self {
            board,
            link,
            hcell: hcell as i32,
            wcell: wcell as i32,
            anim_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => false,
            Msg::MakeMove => {
                let pts = self
                    .board
                    .pieces
                    .iter()
                    .filter(|(p, c)| **c == self.board.turn)
                    .collect::<Vec<_>>();
                let pt = pts.choose(&mut rand::thread_rng());
                if let Some(pt) = pt {
                    let moves = self.board.get_legal_moves(pt.0);
                    let mv = moves.choose(&mut rand::thread_rng());
                    if let Some(mv) = mv {
                        self.board.apply_move(&Move {
                            from: *pt.0,
                            to: *mv,
                        });
                        self.board.change_turn(self.get_next_color());
                    } else {
                        self.board = Self::generate_board().0;
                    }
                }
                self.anim_agent.send(AgentInput::Reset);
                self.anim_agent.send(AgentInput::Forward);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let cellwidth = 100.0 / ((self.wcell * 2) - 1) as f32;
        let cellheight: f32 = 100.0 / ((self.hcell * 2) - 1) as f32;
        let hexs = self.board.points.iter().map(|(k, v)| {
            let off = OffsetCoord::from(v.clone());
            let shift_left = {
                if off.row % 2 != 0 {
                    0.5
                } else {
                    0 as f32
                }
            };
            let shift_top = { off.row as f32 * cellheight * 0.25 };
            let pt = k.clone();
            // log::debug!("width {}, height {} ", cellwidth, cellheight);
            let mut color = GameColors::NormalCellColor;
            let mut piece = None;
            if let Some(val) = self.board.pieces.get(&pt) {
                piece = Some(*val);
            }
            html! {
                <>
                <div class="hexagon"
                style=format!(
                    "height:{}%;width:{}%;
                    
                    left:{}%;top:{}%;
                    ",

                    cellheight,
                    cellwidth,
                    cellwidth*(off.col - 1 + self.wcell as i32) as f32 + shift_left*cellwidth,
                    cellheight*(off.row - 1 + self.hcell as i32) as f32 - shift_top,
                )
                onclick = self.link.callback(move|_|Msg::Ignore)
                >
                    <div class="hex-cell"
                        style = format!(
                            "background-color:{};",
                            piece.map(|f|String::from(f)).unwrap_or(String::from(color))
                        )
                    />

                </div>


                </>
            }
        });
        html! {
            <div class="honeyback" >
            {
                for hexs
            }
            </div>
        }
    }
}
