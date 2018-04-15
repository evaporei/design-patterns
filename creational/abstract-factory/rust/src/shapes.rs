pub trait Shape {
    fn draw(&self);
}

pub struct Rectangle {}

impl Shape for Rectangle {
    fn draw(&self) {
        println!("Inside Rectangle::draw method.");
    }
}

pub struct Circle {}

impl Shape for Circle {
    fn draw(&self) {
        println!("Inside Circle::draw method.");
    }
}

pub struct Square {}

impl Shape for Square {
    fn draw(&self) {
        println!("Inside Square::draw method.");
    }
}

