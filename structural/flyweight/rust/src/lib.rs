pub trait Shape {
    fn draw(&self);
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
    White,
    Black,
}

use std::fmt;

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Color::*;
        match *self {
            Red => write!(f, "Red"),
            Green => write!(f, "Green"),
            Blue => write!(f, "Blue"),
            White => write!(f, "White"),
            Black => write!(f, "Black"),
        }
    }
}

pub struct Circle {
    color: Color,
    id: isize,
}

impl Circle {
    fn new(id: isize, color: Color) -> Self {
        Circle { color, id }
    }
    pub fn id(&self) -> isize {
        self.id
    }
}

impl Shape for Circle {
    fn draw(&self) {
        println!("Circle: Draw() [color: {}, id: {}]", self.color, self.id);
    }
}

use std::collections::HashMap;

pub struct CircleCache {
    circles: HashMap<Color, Circle>,
}

impl CircleCache {
    pub fn new() -> Self {
        CircleCache {
            circles: HashMap::new(),
        }
    }
    pub fn get_circle(&mut self, id: isize, color: Color) -> &Circle {
        self.circles.entry(color.clone()).or_insert(Circle::new(id, color))
    }
}
