pub mod shapes;
pub mod colors;

use shapes::*;
use colors::*;

pub trait AbstractFactory {
    fn get_color(&self, name: &str) -> Option<Box<Color>>;
    fn get_shape(&self, name: &str) -> Option<Box<Shape>>;
}

pub struct ShapeFactory {}

impl AbstractFactory for ShapeFactory {
    fn get_color(&self, _name: &str) -> Option<Box<Color>> {
        None
    }
    fn get_shape(&self, name: &str) -> Option<Box<Shape>> {
        match name {
            "RECTANGLE" => Some(Box::new(Rectangle {})),
            "CIRCLE" => Some(Box::new(Circle {})),
            "SQUARE" => Some(Box::new(Square {})),
            _ => None,
        }
    }
}

pub struct ColorFactory {}

impl AbstractFactory for ColorFactory {
    fn get_color(&self, name: &str) -> Option<Box<Color>> {
        match name {
            "RED" => Some(Box::new(Red {})),
            "GREEN" => Some(Box::new(Green {})),
            "BLUE" => Some(Box::new(Blue {})),
            _ => None,
        }
    }
    fn get_shape(&self, _name: &str) -> Option<Box<Shape>> {
        None
    }
}

pub struct FactoryProducer {}

impl FactoryProducer {
    pub fn get_factory(choice: &str) -> Option<Box<AbstractFactory>> {
        match choice {
            "SHAPE" => Some(Box::new(ShapeFactory {})),
            "COLOR" => Some(Box::new(ColorFactory {})),
            _ => None,
        }
    }
}

