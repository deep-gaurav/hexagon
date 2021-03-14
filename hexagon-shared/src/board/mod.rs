use std::{collections::HashMap, hash::Hash, ops::Index};

use crate::{colors::colors::Color, models::*, structures::Move};
use itertools::Itertools;
use rand::seq::SliceRandom;

use serde::{Deserialize, Serialize};

pub type Point = (i32, i32);

impl From<Point> for AxialCoord {
    fn from(p: Point) -> Self {
        AxialCoord { q: p.0, r: p.1 }
    }
}

impl From<AxialCoord> for Point {
    fn from(p: AxialCoord) -> Self {
        (p.q, p.r)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub points: HashMap<Point, AxialCoord>,
    pub max_size: u32,
    pub turn: Color,
    pub pieces: HashMap<Point, Color>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Board {
    pub fn generate_hexagon(size: u32, first_turn: Color, second_color: Color) -> Self {
        let mut points = HashMap::new();

        for axis1 in vec![Axis::X, Axis::Y, Axis::Z] {
            for iu in 0..size as i32 {
                for i in [-iu, iu].iter() {
                    for axis2 in vec![Axis::X, Axis::Y, Axis::Z] {
                        if axis1 == axis2 {
                            continue;
                        }
                        for j in 0..size as i32 {
                            fn get_val_for_axis(
                                axis1: Axis,
                                axis2: Axis,
                                i: i32,
                                j: i32,
                                cur_axis: Axis,
                            ) -> i32 {
                                if cur_axis == axis1 {
                                    i
                                } else if cur_axis == axis2 {
                                    j
                                } else {
                                    -j - i
                                }
                            }
                            let cube = Cube {
                                x: get_val_for_axis(axis1, axis2, *i, j, Axis::X),
                                y: get_val_for_axis(axis1, axis2, *i, j, Axis::Y),
                                z: get_val_for_axis(axis1, axis2, *i, j, Axis::Z),
                            };
                            if cube.x * cube.x >= (size * size) as i32 {
                                continue;
                            }
                            if cube.y * cube.y >= (size * size) as i32 {
                                continue;
                            }
                            if cube.z * cube.z >= (size * size) as i32 {
                                continue;
                            }

                            let axial = AxialCoord::from(cube);
                            let point = (axial.q, axial.r);
                            points.entry(point).or_insert(axial);
                        }
                    }
                }
            }
        }

        let mut pieces = HashMap::new();
        for i in [0, 1, 2].into_iter() {
            let c1 = *i;
            let c2 = (*i + 1) % 3;
            let c3 = (*i + 2) % 3;

            let size = size - 1;
            pieces
                .entry(
                    AxialCoord::from(Cube {
                        x: if c1 == 0 {
                            size as i32
                        } else if c1 == 1 {
                            -(size as i32)
                        } else {
                            0
                        },
                        y: if c2 == 0 {
                            size as i32
                        } else if c2 == 1 {
                            -(size as i32)
                        } else {
                            0
                        },
                        z: if c3 == 0 {
                            size as i32
                        } else if c3 == 1 {
                            -(size as i32)
                        } else {
                            0
                        },
                    })
                    .into(),
                )
                .or_insert(first_turn);
            pieces
                .entry(
                    AxialCoord::from(Cube {
                        x: if c1 == 0 {
                            size as i32
                        } else if c1 == 2 {
                            -(size as i32)
                        } else {
                            0
                        },
                        y: if c2 == 0 {
                            size as i32
                        } else if c2 == 2 {
                            -(size as i32)
                        } else {
                            0
                        },
                        z: if c3 == 0 {
                            size as i32
                        } else if c3 == 2 {
                            -(size as i32)
                        } else {
                            0
                        },
                    })
                    .into(),
                )
                .or_insert(second_color);
        }
        Self {
            points,
            max_size: size,
            turn: first_turn,
            pieces,
        }
    }

    pub fn generate_honeycomb(
        width: i32,
        height: i32,
        fill_per_color: usize,
        first_turn: Color,
        second_color: Color,
    ) -> Self {
        let mut points = HashMap::new();
        for i in -width..width {
            for j in -height - 2..height + 2 {
                let ax = AxialCoord::from(Cube::from(OffsetCoord { row: j, col: i }));
                points.insert((ax.q, ax.r), ax);
            }
        }
        let random_colors = 5;

        let mut pieces = HashMap::new();
        for i in 0..fill_per_color {
            let colors = vec![first_turn, second_color];
            for color in colors {
                let rp = points
                    .iter()
                    .filter(|(p, c)| !pieces.contains_key(*p))
                    .collect_vec();
                let rp = rp.choose(&mut rand::thread_rng());
                if let Some(p) = rp {
                    pieces.insert(*p.0, color);
                }
            }
        }
        Self {
            points,
            pieces,
            turn: first_turn,
            max_size: width as u32,
        }
    }

    pub fn get_neighbours(&self, point: &Point) -> Vec<Point> {
        let cube_directions = [
            Cube { x: 1, y: -1, z: 0 },
            Cube { x: 1, y: 0, z: -1 },
            Cube { x: 0, y: 1, z: -1 },
            Cube { x: -1, y: 1, z: 0 },
            Cube { x: -1, y: 0, z: 1 },
            Cube { x: 0, y: -1, z: 1 },
        ];
        let mut neighbours = vec![];
        if let Some(pt) = self.points.get(&point) {
            let cb = Cube::from(pt.clone());
            for dir in cube_directions.iter() {
                let neighbour: Cube = cb.clone() + dir.clone();
                let neighbour_ax = AxialCoord::from(neighbour);
                let pt = (neighbour_ax.q, neighbour_ax.r);
                if self.points.contains_key(&pt) {
                    neighbours.push(pt);
                }
            }
        }
        neighbours
    }

    pub fn get_secondary_neighbours(&self, point: &Point) -> Vec<Point> {
        let mut secondaryneighbours = vec![];
        let neighbourpts = self.get_neighbours(point);
        for pts in neighbourpts.iter() {
            secondaryneighbours.append(&mut self.get_neighbours(pts));
        }
        secondaryneighbours
    }

    pub fn is_move_legal(&self, mov: &Move) -> bool {
        if let Some(piece) = self.pieces.get(&mov.from) {
            if *piece != self.turn {
                false
            } else {
                let secondaryneighbours = self.get_secondary_neighbours(&mov.from);
                if secondaryneighbours.contains(&mov.to) {
                    if mov.to == mov.from {
                        false
                    } else {
                        if self.pieces.get(&mov.to).is_some() {
                            false
                        } else {
                            true
                        }
                    }
                } else {
                    false
                }
            }
        } else {
            false
        }
    }

    pub fn get_legal_moves(&self, pt: &Point) -> Vec<Point> {
        let secn = self.get_secondary_neighbours(pt);
        secn.into_iter()
            .filter(|m| self.is_move_legal(&Move { from: *pt, to: *m }))
            .collect()
    }

    pub fn apply_move(&mut self, mov: &Move) -> bool {
        if self.is_move_legal(mov) {
            let neighbour = self.get_neighbours(&mov.from);
            if neighbour.contains(&mov.to) {
                self.pieces.insert(mov.to, self.turn);
                let neighours = self.get_neighbours(&mov.to);
                for point in neighours.iter() {
                    if self.pieces.get(point).is_some() {
                        self.pieces.insert(*point, self.turn);
                    }
                }
                true
            } else if self.get_secondary_neighbours(&mov.from).contains(&mov.to) {
                self.pieces.remove(&mov.from);
                self.pieces.insert(mov.to, self.turn);
                let neighours = self.get_neighbours(&mov.to);
                for point in neighours.iter() {
                    if self.pieces.get(point).is_some() {
                        self.pieces.insert(*point, self.turn);
                    }
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    pub fn change_turn(&mut self, next_color: Color) {
        self.turn = next_color;
    }
}
