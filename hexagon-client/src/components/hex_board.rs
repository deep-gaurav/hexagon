use yew::prelude::*;

use hexagon_shared::{board::{Board, Point}, colors::colors::Color};
use hexagon_shared::models::*;

pub struct HexBoard {
    pub board: Board,
    selected_cell: Option<Point>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    SelectPoint(Point),
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub board:Board
}

impl Component for HexBoard {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            board: props.board,
            selected_cell: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SelectPoint(pt) => {
                if let Some(ptold) = self.selected_cell {
                    if pt == ptold {
                        self.selected_cell = None;
                    } else {
                        self.selected_cell = Some(pt);
                    }
                } else {
                    self.selected_cell = Some(pt);
                }
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let cellwidth = 100.0 / ((self.board.max_size * 2) - 1) as f32;
        let neighbourpts = {
            if let Some(pt) = self.selected_cell {
                self.board.get_neighbours(&pt)
            } else {
                vec![]
            }
        };
        let mut secondaryneighbours = vec![];
        for pts in neighbourpts.iter(){
            secondaryneighbours.append(
                &mut self.board.get_neighbours(pts)
            );
        }
        let hexs = self.board.points.iter().map(|(k,v)|{
            let off = OffsetCoord::from(v.clone());
            let shift_left = {
                if off.row%2 !=0 {
                    0.5
                }else{
                    0 as f32
                }
            };
            let shift_top = {
                off.row as f32 * cellwidth * 0.25
            };
            let pt = k.clone();

            let color = {
                if let Some(cell)= self.selected_cell{
                    if cell == pt{
                        Color::Red
                    }else{
                        if neighbourpts.contains(&pt){
                            Color::DarkRed
                        }else if secondaryneighbours.contains(&pt){
                            Color::LightRed
                        }else{
                            Color::Blue
                        }
                    }
                }else{
                    Color::Blue
                }
            };
            html!{
                <div class="hexagon" 
                style=format!(
                    "height:{}%;width:{}%;
                    
                    left:{}%;top:{}%;
                    background-color:{};
                    ",
                    cellwidth,
                    cellwidth,
                    cellwidth*(off.col - 1 + self.board.max_size as i32) as f32 + shift_left*cellwidth,
                    cellwidth*(off.row - 1 + self.board.max_size as i32) as f32 - shift_top,
                    String::from(color)
                )
                onclick = self.link.callback(move|_|Msg::SelectPoint(pt.clone()))
                >
                    // {
                    //     format!("x:{},y:{}",off.col,off.row)
                    // }
                </div>
            }
        });
        html! {
            <div class="hex-board" >
            {
                for hexs
            }
            </div>
        }
    }
}
