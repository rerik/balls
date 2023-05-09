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
}
