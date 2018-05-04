trait Shape {
    fn draw(&self);
}

pub struct Circle;

impl Shape for Circle {
    fn draw(&self) {
        println!("Circle::draw");
    }
}

pub struct Rectangle;

impl Shape for Rectangle {
    fn draw(&self) {
        println!("Rectangle::draw");
    }
}

pub struct Square;

impl Shape for Square {
    fn draw(&self) {
        println!("Square::draw");
    }
}

pub struct ShapeMaker {
    circle: Circle,
    rectangle: Rectangle,
    square: Square,
}

impl ShapeMaker {
    pub fn new(circle: Circle, rectangle: Rectangle, square: Square) -> Self {
        ShapeMaker { circle, rectangle, square }
    }
    pub fn draw_circle(&self) {
        self.circle.draw();
    }
    pub fn draw_rectangle(&self) {
        self.rectangle.draw();
    }
    pub fn draw_square(&self) {
        self.square.draw();
    }
}
