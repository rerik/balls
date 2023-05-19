// extern crate piston_window;

// use piston_window::*;

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

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    ball: Ball,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        // const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {

            // Clear the screen.
            clear(BLACK, gl);

            self.ball.draw(c, gl);
        });
        
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.ball.mov(args.dt);
        // println!("Speed {}", self.ball.speed);
        if self.ball.coords.x > 100. {
            self.ball.speed.reflect(Vec2 { x: 0., y: 1. }, 1.);
        }
    }
}

fn main() {
    // Vec2 tests
    let mut place: Vec2 = Vec2::new(5., 7.);
    println!("Place 1: {}", place);

    let shift: Vec2 = Vec2::new(1., 3.);

    place += shift;
    println!("Place 2: {}", place);
    println!("Place 3: {}", place * 2.);

    //Ball tests
    let mut b: Ball = Ball::new(
        Vec2::new(3., 4.), 
        3., 
        Vec2::new(1., 5.), 
        5.
    );
    println!("Ball 1: {}", b);

    b.relocate(Vec2::new(2., 2.));
    println!("Ball 2: {}", b);

    b.mov(1.);
    println!("Ball 3: {}", b);

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        ball: Ball::new(
            Vec2::new(5., 5.), 
            5., 
            Vec2::new(30., 10.), 
            5.
        ),
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
    // }
}
