extern crate factory;

use factory::*;

fn main() {
    let shape_factory = ShapeFactory {};

    let shape1 = shape_factory.get_shape("RECTANGLE");

    if let Some(rectangle) = shape1 {
        rectangle.draw();
    }

    let shape2 = shape_factory.get_shape("CIRCLE");

    if let Some(circle) = shape2 {
        circle.draw();
    }

    let shape3 = shape_factory.get_shape("SQUARE");

    if let Some(square) = shape3 {
        square.draw();
    }
}
