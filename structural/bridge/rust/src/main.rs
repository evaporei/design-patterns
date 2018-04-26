extern crate bridge;

use bridge::Shape;
use bridge::Circle;
use bridge::RedCircle;
use bridge::GreenCircle;

fn main() {
    let red_circle = Circle::new(10, 100, 100, Box::new(RedCircle));
    let green_circle = Circle::new(10, 100, 100, Box::new(GreenCircle));

    red_circle.draw();
    green_circle.draw();
}
