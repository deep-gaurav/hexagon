use yew::prelude::*;

use hexagon_shared::{board::{Board, Point}, colors::colors::Color, structures::Move};
use hexagon_shared::models::*;

use crate::ui::GameColors;

pub struct HexBoard {
    pub board: Board,
    pub is_sim:bool,
    move_callback:Callback<Move>,
    selected_cell: Option<Point>,
    player_color:Color,
    link: ComponentLink<Self>,
}

pub enum Msg {
    SelectPoint(Point),
    
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub board:Board,
    pub color:Color,
    pub move_callback:Callback<Move>,
    pub is_sim:bool,
}

impl Component for HexBoard {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            board: props.board,
            selected_cell: None,
            link,
            player_color:props.color,
            move_callback:props.move_callback,
            is_sim:props.is_sim
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SelectPoint(pt) => {
                if self.is_sim{
                    false
                }else{
                    if let Some(ptold) = self.selected_cell {
                        if self.board.is_move_legal(&Move{
                            from:ptold,
                            to:pt,
                        }){
                            self.move_callback.emit(Move{
                                from:ptold,
                                to:pt,
                            });
                        }
                        else if pt == ptold {
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
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let cellwidth = 100.0 / ((self.board.max_size * 2) - 1) as f32;
        
        let mut neighbourpts = vec![];
        let mut secondaryneighbours = vec![];
        
        if let Some(pt) = self.selected_cell {
            if let Some(c) = self.board.pieces.get(&pt){
                if *c == self.board.turn && self.board.turn == self.player_color{
                    neighbourpts= self.board.get_neighbours(&pt);
                    secondaryneighbours = self.board.get_secondary_neighbours(&pt);
                }
            }
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

            let mut color = {
                if let Some(cell)= self.selected_cell{
                    if cell == pt{
                        GameColors::SelectedCellColor
                    }else{
                        if neighbourpts.contains(&pt){
                            GameColors::NearNeighbourColor
                        }else if secondaryneighbours.contains(&pt){
                            GameColors::FarNeighbourColor
                        }else{
                            GameColors::NormalCellColor
                        }
                    }
                }else{
                    GameColors::NormalCellColor
                }
            };
            let mut piece = None;
            if let Some(val)= self.board.pieces.get(&pt){
                piece = Some(*val);
            }
            html!{
                <>
                <div class="hexagon" 
                style=format!(
                    "height:{}%;width:{}%;
                    
                    left:{}%;top:{}%;
                    ",
                    cellwidth,
                    cellwidth,
                    cellwidth*(off.col - 1 + self.board.max_size as i32) as f32 + shift_left*cellwidth,
                    cellwidth*(off.row - 1 + self.board.max_size as i32) as f32 - shift_top,
                    
                )
                onclick = self.link.callback(move|_|Msg::SelectPoint(pt.clone()))
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
            <div class="hex-board" >
            {
                for hexs
            }
            </div>
        }
    }
}
