#![recursion_limit="1024"]
use yew::prelude::*;

use components::hex_board::HexBoard;

pub mod components;
pub mod agent;
pub mod app;
pub mod ui;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
}
