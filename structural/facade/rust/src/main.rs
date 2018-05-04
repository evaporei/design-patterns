extern crate facade;

use facade::ShapeMaker;
use facade::Circle;
use facade::Rectangle;
use facade::Square;

fn main() {
    let shape_maker = ShapeMaker::new(Circle, Rectangle, Square);

    shape_maker.draw_circle();
    shape_maker.draw_rectangle();
    shape_maker.draw_square();
}
