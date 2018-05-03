pub trait Shape {
    fn draw(&self);
}

pub struct Rectangle;

impl Shape for Rectangle {
    fn draw(&self) {
        println!("Shape: Rectangle");
    }
}

pub struct Circle;

impl Shape for Circle {
    fn draw(&self) {
        println!("Shape: Circle");
    }
}

pub trait ShapeDecorator {
    fn decorated_shape(&self) -> &Box<Shape>;
    fn draw(&mut self) {
        self.decorated_shape().draw();
    }
}

pub struct RedShapeDecorator {
    decorated_shape: Box<Shape>,
}

impl ShapeDecorator for RedShapeDecorator {
    fn decorated_shape(&self) -> &Box<Shape> {
        &self.decorated_shape
    }
    fn draw(&mut self) {
        self.decorated_shape().draw();
        self.set_red_border();
    }
}

impl RedShapeDecorator {
    pub fn new(shape: Box<Shape>) -> Self {
        RedShapeDecorator {
            decorated_shape: shape,
        }
    }
    fn set_red_border(&self) {
        println!("Border Color: Red");
    }
}
