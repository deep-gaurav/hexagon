use yew::prelude::*;

use components::hex_board::HexBoard;

pub mod components;

pub struct App{

}

#[derive(Debug,Clone,Properties,Default)]
pub struct Props{
    
}

pub enum Msg {}

impl Component for App {
    type Message =Msg;

    type Properties=Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self{}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!{
            <div>
                <HexBoard/>
            </div>
        }
    }
}


fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
