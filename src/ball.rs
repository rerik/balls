use std::fmt;

use crate::vec2::Vec2;

extern crate graphics;
// use graphics::*;

pub struct Ball {
    coords: Vec2,
    size: f64,
    speed:  Vec2,
    mass: f64,
}

impl Ball {
    pub fn new(coords: Vec2, size: f64, speed: Vec2, mass: f64) -> Self {
        Self {
            coords,
            size,
            speed,
            mass,
        }
    }
    pub fn relocate(&mut self, shift: Vec2) {
        self.coords += shift;
    }
    pub fn mov(&mut self, time: f64) {
        self.coords += self.speed * time;
    }
    pub fn rectangle(&mut self) -> [f64; 4] {
        graphics::rectangle::square(self.coords.x, self.coords.y, self.size/2.)
    }
}

impl fmt::Display for Ball {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "
    coords: {}
    size: {}
    speed: {}
    mass: {}
",
            self.coords, 
            self.size, 
            self.speed,
            self.mass
        )
    }
}
