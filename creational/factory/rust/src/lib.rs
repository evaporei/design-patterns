pub trait Shape {
    fn draw(&self);
}

struct Rectangle {}

impl Shape for Rectangle {
    fn draw(&self) {
        println!("Inside Rectangle::draw method.");
    }
}

struct Circle {}

impl Shape for Circle {
    fn draw(&self) {
        println!("Inside Circle::draw method.");
    }
}

struct Square {}

impl Shape for Square {
    fn draw(&self) {
        println!("Inside Square::draw method.");
    }
}

pub struct ShapeFactory {}

impl ShapeFactory {
    pub fn get_shape(&self, name: &str) -> Option<Box<Shape>> {
        match name {
            "RECTANGLE" => Some(Box::new(Rectangle {})),
            "CIRCLE" => Some(Box::new(Circle {})),
            "SQUARE" => Some(Box::new(Square {})),
            _ => None,
        }
    }
}
