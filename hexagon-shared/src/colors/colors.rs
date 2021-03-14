use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(
    Debug, EnumIter, PartialEq, Clone, Copy, Serialize, Deserialize, Eq, Hash, PartialOrd, Ord,
)]
pub enum Color {
    Green,
    Red,
    DarkRed,
    LightRed,
    Blue,
    Yellow,
    Transparent,
    BackgroundP1,
    BackgroundP2,
}

impl From<Color> for String {
    fn from(color: Color) -> Self {
        match color {
            Color::Green => "green".to_string(),
            Color::Red => "red".to_string(),
            Color::DarkRed => "darkred".to_string(),
            Color::Blue => "blue".to_string(),
            Color::Yellow => "yellow".to_string(),
            Color::LightRed => "lightred".to_string(),
            Color::Transparent => "transparent".to_string(),
            Color::BackgroundP1 => {"var(--background-player1)".to_string()}
            Color::BackgroundP2 => {"var(--background-player2)".to_string()}
        }
    }
}
