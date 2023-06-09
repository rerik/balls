use std::fmt;

use crate::vec2::Vec2;

extern crate graphics;
use graphics::*;
extern crate opengl_graphics;
use opengl_graphics::GlGraphics;
// use piston_window::{draw_state, Context};

pub struct Ball {
    coords: Vec2,
    size: f64,
    speed:  Vec2,
    mass: f64,
    circle: circle_arc::CircleArc,
}

impl Ball {
    pub fn new(coords: Vec2, size: f64, speed: Vec2) -> Self {
        Self {
            coords,
            size,
            speed,
            mass: size*size,
            circle: circle_arc::CircleArc::new([1.0; 4], size, 0.0, f64::_360())
        }
    }
    pub fn mov(&mut self, time: f64) {
        self.coords += self.speed * time;
    }
    pub fn rectangle(& self) -> [f64; 4] {
        graphics::rectangle::square(self.coords.x, self.coords.y, self.size)
    }
    pub fn draw(& self, c: Context, g: &mut GlGraphics) {
        self.circle.draw(self.rectangle(), &c.draw_state, c.transform, g);
    }
    pub fn reflect(&mut self, line: Vec2) {
        self.speed.reflect(line);
    }
    pub fn check_out_of_scope(&mut self, width: u32, height: u32) {
        if self.coords.x + self.size*2. > width as f64 && self.speed.x > 0. {
            self.reflect(Vec2 { x: 0., y: 1. });
        }
        if self.coords.x - self.size < 0. && self.speed.x < 0. {
            self.reflect(Vec2 { x: 0., y: 1. });
        }
        if self.coords.y + self.size*2. > height as f64 && self.speed.y > 0. {
            self.reflect(Vec2 { x: 1., y: 0. });
        }
        if self.coords.y - self.size < 0. && self.speed.y < 0. {
            self.reflect(Vec2 { x: 1., y: 0. });
        }
    }
    pub fn collide(&mut self, other: &mut Self) {
        let distance: Vec2 = self.coords - other.coords;
        if distance.len() <= (self.size + other.size)*1.4 {
            let mass_coef: f64 = 2. / (self.mass + (*other).mass);
            let speed_diff: Vec2 = self.speed - other.speed;
            let new_self_speed: Vec2 = self.speed - (distance * (mass_coef*other.mass*(speed_diff*distance)/distance.len_sqr()));
            let neg_distance: Vec2 = -distance;
            let new_other_speed: Vec2 = other.speed - (neg_distance * (mass_coef*self.mass*((-speed_diff)*neg_distance)/neg_distance.len_sqr()));
            self.speed = new_self_speed;
            other.speed = new_other_speed;
        }
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
