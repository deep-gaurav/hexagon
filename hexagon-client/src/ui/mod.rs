#[derive(Debug,Clone, Copy)]
pub enum GameColors{
    NormalCellColor,
    SelectedCellColor,
    NearNeighbourColor,
    FarNeighbourColor,
}

impl From<GameColors> for String {
    fn from(color: GameColors) -> Self {
        match color {
            GameColors::NormalCellColor => {
                "var(--noarmalCellColor)".into()
            }
            GameColors::SelectedCellColor => {
                "var(--selectedCellColor)".into()
            }
            GameColors::NearNeighbourColor => {
                "var(--nearNeighbourColor)".into()
            }
            GameColors::FarNeighbourColor => {
                "var(--farNeighbourColor)".into()
            }
        }
    }
}