use std::fmt;

use crate::vec2::Vec2;

pub struct Ball {
    coords: Vec2,
    size: f32,
    speed: f32,
    direction: Vec2,
    mass: f32,
}

impl Ball {
    pub fn new(coords: Vec2, size: f32, speed: f32, direction: Vec2, mass: f32) -> Self {
        Self {
            coords,
            size,
            speed,
            direction,
            mass,
        }
    }
    pub fn replace(&mut self, shift: Vec2) {
        self.coords += shift;
    }
    pub fn mov(&mut self, time: f32) {
        self.coords += self.direction * (self.speed * time);
    }
}

impl fmt::Display for Ball {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "
    coords: {},
    size: {},
    speed: {},
    direction: {},
    mass: {}
",
            self.coords, 
            self.size, 
            self.speed, 
            self.direction, 
            self.mass
        )
    }
}
