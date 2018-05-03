extern crate decorator;

use decorator::Shape;
use decorator::Circle;
use decorator::Rectangle;
use decorator::ShapeDecorator;
use decorator::RedShapeDecorator;

fn main() {
    let circle = Circle;

    let mut red_circle = RedShapeDecorator::new(Box::new(Circle));

    let mut red_rectangle = RedShapeDecorator::new(Box::new(Rectangle));

    println!("Circle with normal border");
    circle.draw();

    println!("\nCircle with red border");
    red_circle.draw();

    println!("\nRectangle with red border");
    red_rectangle.draw();
}
