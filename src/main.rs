// extern crate piston_window;

// use piston_window::*;

use array_macro::array;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod vec2;
use vec2::Vec2;

mod ball;
use ball::Ball;

mod range;
use range::Range;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 450;

const BALLS_NUMBER: usize = 5;

const SIZE: Range = Range::new(2., 50.);
const COORD_X: Range = Range::new(SIZE.max, WIDTH as f64 - SIZE.max);
const COORD_Y: Range = Range::new(SIZE.max, HEIGHT as f64 - SIZE.max);
const SPEED: Range = Range::new(10., 500.);
// const SPEED: Range = Range::new(10., 50.);
const MASS: Range = Range::new(1., 10.);

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    balls: [Ball; BALLS_NUMBER],
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        // const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        // const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {

            // Clear the screen.
            clear(BLACK, gl);

            for ball in self.balls.iter() {
                ball.draw(c, gl);
            }
            
        });
        
    }

    fn update(&mut self, args: &UpdateArgs) {
        for ball in self.balls.iter_mut() {
            ball.mov(args.dt);
            ball.check_out_of_scope(WIDTH, HEIGHT);
        }
        for i in 0..BALLS_NUMBER {
            let (start_balls, end_balls) = self.balls.split_at_mut(i+1);
            for j in 0..BALLS_NUMBER-i-1 {
                start_balls[i].collide(&mut end_balls[j]);
            }
        }  
    }
}

fn main() {

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl), 
        balls: array![_ => Ball::new(
                Vec2::new(COORD_X.gen(), COORD_Y.gen()), 
                SIZE.gen(), 
                Vec2::new(SPEED.gen(), SPEED.gen()), 
                MASS.gen(),
        ); BALLS_NUMBER]
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }

}
