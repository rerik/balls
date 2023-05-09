use std::fmt;

use crate::vec2::Vec2;

pub struct Ball {
    coords: Vec2,
    size: f32,
    speed:  Vec2,
    mass: f32,
}

impl Ball {
    pub fn new(coords: Vec2, size: f32, speed: Vec2, mass: f32) -> Self {
        Self {
            coords,
            size,
            speed,
            mass,
        }
    }
    pub fn replace(&mut self, shift: Vec2) {
        self.coords += shift;
    }
    pub fn mov(&mut self, time: f32) {
        self.coords += self.speed * time;
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
