extern crate piston_window;

use piston_window::*;


mod vec2;
use vec2::Vec2;

mod ball;
use ball::Ball;

fn main() {
    // Vec2 tests
    let mut place: Vec2 = Vec2::new(5., 7.);
    println!("Place 1: {}", place);

    let shift: Vec2 = Vec2::new(1., 3.);

    place += shift;
    println!("Place 2: {}", place);

    //Ball tests
    let mut b: Ball = Ball::new(
        Vec2::new(3., 4.), 
        3., 
        Vec2::new(1., 1.), 
        5.
    );
    println!("Ball 1: {}", b);

    b.replace(Vec2::new(2., 2.));
    println!("Ball 2: {}", b);

    b.mov(1.);
    println!("Ball 3: {}", b);

    let mut window: PistonWindow = 
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    let mut x_pos = 30.0;
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _device| {
            clear([0.0; 4], g);
            // rectangle([1.0, 0.0, 0.0, 1.0], // red
            //           [0.0, 0.0, 100.0, 100.0],
            //           c.transform, g);
            circle_arc([1.0; 4], 15.0, 0.0, f64::_360(), [230.0, x_pos, 30.0, 30.0], c.transform, g)
        });
        x_pos += 0.1;
        // let circle = CircleArc::new([1.0; 4], 10.0, 0.0, f64::_360());
        // circle.draw([10.0, 10.0, 20.0, 20.0], &e, Context.transform, g);
    }
}
