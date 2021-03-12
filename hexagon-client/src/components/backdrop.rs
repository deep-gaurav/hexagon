use hexagon_shared::{board::Board, colors::colors::Color, models::OffsetCoord};
use yew::prelude::*;
use crate::components::hex_board::HexBoard;
use crate::ui::GameColors;

pub struct HoneyCombBackdrop{
    pub board:Board,
    pub link:ComponentLink<Self>,
    pub wcell:i32,
    pub hcell:i32,
}

pub enum Msg{
    Ignore
}

#[derive(Debug,Clone,Properties)]
pub struct Props{

}

impl Component for HoneyCombBackdrop {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let width:f32 = yew::utils::window().inner_width().expect("Width not present").as_f64().expect("Not number") as f32;
        let height:f32 = yew::utils::window().inner_height().expect("height not present").as_f64().expect("Not number") as f32;
        let height_ratio = width/height;

        let hex_r = 1.1547005;
        let cellwidth = 100.0;
            let wcell = width/cellwidth;
            let hcell = height/cellwidth *hex_r ;
        let board = {
                Board::generate_honeycomb(wcell as i32,hcell as i32, Color::Green, Color::Red)
            
        };
        Self{
            board,
            link,
            hcell: hcell as i32,
            wcell:wcell as i32
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let cellwidth = 100.0 / ((self.wcell * 2) -1) as f32;
        let cellheight:f32 = 100.0 / ((self.hcell * 2) -1) as f32;
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
                off.row as f32 * cellheight * 0.25
            };
            let pt = k.clone();
            log::debug!("width {}, height {} ", cellwidth, cellheight);
            let mut color = GameColors::NormalCellColor;
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
                    background-color:{};
                    ",
                    cellheight,
                    cellwidth,
                    cellwidth*(off.col - 1 + self.wcell as i32) as f32 + shift_left*cellwidth,
                    cellheight*(off.row - 1 + self.hcell as i32) as f32 - shift_top,
                    String::from(color)
                )
                onclick = self.link.callback(move|_|Msg::Ignore)
                >
                    
                </div>
                {
                    if let Some(piece)=piece{
                        html!{
                            <div class="piece" 
                            style=format!(
                                "height:calc({}% - 10px);width:calc({}% - 10px);
                                
                                left:calc({}% + 5px);top:calc({}% + 5px);
                                background-color:{};
                                ",
                                cellwidth,
                                cellwidth,
                                cellwidth*(off.col - 1 + self.board.max_size as i32) as f32 + shift_left*cellwidth,
                                cellwidth*(off.row - 1 + self.board.max_size as i32) as f32 - shift_top,
                                String::from(piece)
                            )
                            onclick = self.link.callback(move|_|Msg::Ignore)
                            >
                                
                            </div>
                        }
                    }else{
                        html!{
                            
                        }
                    }
                }
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