use std::collections::HashMap;

use crate::{colors::colors::Color, models::{*}};

use serde::{Serialize,Deserialize};

pub type Point = (i32,i32);

impl From<Point> for AxialCoord {
    fn from(p: Point) -> Self {
        AxialCoord{
            q:p.0,
            r:p.1,
        }
    }
}

impl From<AxialCoord> for Point {
    fn from(p: AxialCoord) -> Self {
        (p.q,p.r)
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Board{
    pub points:HashMap<Point,AxialCoord>,
    pub max_size:u32,
    pub turn:Color
}

#[derive(Debug,PartialEq,Clone, Copy)]
pub enum Axis{
    X,
    Y,
    Z
}

impl Board {
    pub fn generate_hexagon(size:u32,first_turn:Color)->Self{
        let mut points = HashMap::new();

        for axis1 in vec![Axis::X,Axis::Y,Axis::Z]{
            for iu in 0..size as i32{
                for i in [-iu,iu].iter(){
                    for axis2 in vec![Axis::X,Axis::Y,Axis::Z]{
                        if axis1==axis2{
                            continue;
                        }
                        for j in 0..size as i32{
                            fn get_val_for_axis(axis1:Axis,axis2:Axis,i:i32,j:i32,cur_axis:Axis)->i32{
                                if cur_axis==axis1{
                                    i
                                }else if cur_axis==axis2 {
                                    j
                                }else{
                                    -j-i
                                }
                            }
                            let cube = Cube{
                                x:get_val_for_axis(axis1, axis2, *i, j, Axis::X),
                                y:get_val_for_axis(axis1, axis2, *i, j, Axis::Y),
                                z:get_val_for_axis(axis1, axis2, *i, j, Axis::Z),
                            };
                            if cube.x*cube.x >= (size*size) as i32{
                                continue;
                            }
                            if cube.y*cube.y >= (size*size) as i32{ 
                                continue;
                            }
                            if cube.z*cube.z >= (size*size) as i32{
                                continue;
                            }
    
                            let axial = AxialCoord::from(cube);
                            let point = (axial.q,axial.r);
                            points.entry(point).or_insert(axial);
                        }
                    }
                }
            }
        }

        Self{
            points,
            max_size:size,
            turn:first_turn
        }
    }

    pub fn get_neighbours(&self,point:&Point)->Vec<Point>{
        let cube_directions = [
            Cube{x:1,y:-1,z:0},
            Cube{x:1,y:0,z:-1}, 
            Cube{x:0,y:1,z:-1}, 
            Cube{x:-1,y:1,z:0}, 
            Cube{x:-1,y:0,z:1}, 
            Cube{x:0,y:-1,z:1},  
        ];
        let mut neighbours = vec![];
        if let Some(pt)= self.points.get(&point){
            let cb = Cube::from(pt.clone());
            for dir in cube_directions.iter(){
                let neighbour:Cube = cb.clone()+dir.clone();
                let neighbour_ax = AxialCoord::from(neighbour);
                let pt = (neighbour_ax.q,neighbour_ax.r);
                if self.points.contains_key(&pt){
                    neighbours.push(pt);
                }
            }
        }
        neighbours
    }
}