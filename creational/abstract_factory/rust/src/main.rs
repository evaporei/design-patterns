extern crate abstract_factory;

use abstract_factory::*;

fn main() {
    let shape_factory_option = FactoryProducer::get_factory("SHAPE");

    if let Some(shape_factory) = shape_factory_option {
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

    let color_factory_option = FactoryProducer::get_factory("COLOR");

    if let Some(color_factory) = color_factory_option {
        let color1 = color_factory.get_color("RED");

        if let Some(red) = color1 {
            red.fill();
        }

        let color2 = color_factory.get_color("GREEN");

        if let Some(green) = color2 {
            green.fill();
        }

        let color3 = color_factory.get_color("BLUE");

        if let Some(blue) = color3 {
            blue.fill();
        }
    }
}
