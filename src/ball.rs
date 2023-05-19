use std::fmt;

use crate::vec2::Vec2;

extern crate graphics;
use graphics::*;
extern crate opengl_graphics;
use opengl_graphics::GlGraphics;
// use piston_window::{draw_state, Context};

pub struct Ball {
    pub coords: Vec2,
    size: f64,
    pub speed:  Vec2,
    mass: f64,
    circle: circle_arc::CircleArc,
}

impl Ball {
    pub fn new(coords: Vec2, size: f64, speed: Vec2, mass: f64) -> Self {
        Self {
            coords,
            size,
            speed,
            mass,
            circle: circle_arc::CircleArc::new([1.0; 4], size, 0.0, f64::_360())
        }
    }
    pub fn relocate(&mut self, shift: Vec2) {
        self.coords += shift;
    }
    pub fn mov(&mut self, time: f64) {
        self.coords += self.speed * time;
    }
    pub fn rectangle(& self) -> [f64; 4] {
        graphics::rectangle::square(self.coords.x, self.coords.y, self.size/2.)
    }
    pub fn draw(& self, c: Context, g: &mut GlGraphics) {
        self.circle.draw(self.rectangle(), &c.draw_state, c.transform, g);
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
