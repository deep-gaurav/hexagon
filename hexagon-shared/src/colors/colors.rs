use strum::EnumIter;
use serde::{Serialize,Deserialize};

#[derive(Debug,EnumIter,PartialEq,Clone,Copy,Serialize,Deserialize,Eq,Hash,PartialOrd, Ord)]
pub enum Color {
    Green,
    Red,
    DarkRed,
    LightRed,
    Blue,
    Yellow,
    Transparent
}

impl From<Color> for String{
    fn from(color: Color) -> Self {
        match color {
            Color::Green => {"green".to_string()}
            Color::Red => {"red".to_string()}
            Color::DarkRed => {"darkred".to_string()}
            Color::Blue => {"blue".to_string()}
            Color::Yellow => {"yellow".to_string()}
            Color::LightRed => {"lightred".to_string()}
            Color::Transparent=> {"transparent".to_string()}
        }
    }
}