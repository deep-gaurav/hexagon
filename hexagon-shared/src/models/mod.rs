use serde::{Serialize,Deserialize};

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct AxialCoord {
    pub q: i32,
    pub r: i32,
}

impl From<Cube> for AxialCoord {
    fn from(cube: Cube) -> Self {
        Self {
            q: cube.x,
            r: cube.z,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cube {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl From<AxialCoord> for Cube {
    fn from(axial: AxialCoord) -> Self {
        Self {
            y: -axial.q - axial.r,
            x: axial.q,
            z: axial.r,
        }
    }
}

impl From<OffsetCoord> for Cube {
    fn from(hex: OffsetCoord) -> Self {
        let x = hex.col - (hex.row - (hex.row&1)) / 2;
        let z = hex.row;
        let y = -x-z;
        Self{
            x,
            y,
            z
        }
    }
}

impl std::ops::Add for Cube{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x:self.x+rhs.x,
            y:self.y+rhs.y,
            z:self.z+rhs.z,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OffsetCoord {
    pub col: i32,
    pub row: i32,
}

impl From<Cube> for OffsetCoord {
    fn from(cube: Cube) -> Self {
        Self{
            col : cube.x + (cube.z - (cube.z&1)) / 2,
            row : cube.z
        }
    }
}

impl From<AxialCoord> for OffsetCoord {
    fn from(ax: AxialCoord) -> Self {
        Self::from(Cube::from(ax))
    }
}