pub trait DrawAPI {
    fn draw_circle(&self, radius: usize, x: usize, y: usize);
}

pub struct RedCircle;

impl DrawAPI for RedCircle {
    fn draw_circle(&self, radius: usize, x: usize, y: usize) {
        println!("Drawing Circle [color: red, radius: {}, x: {}, y: {}]", radius, x, y);
    }
}

pub struct GreenCircle;

impl DrawAPI for GreenCircle {
    fn draw_circle(&self, radius: usize, x: usize, y: usize) {
        println!("Drawing Circle [color: red, radius: {}, x: {}, y: {}]", radius, x, y);
    }
}

pub trait Shape {
    fn draw(&self);
}

pub struct Circle {
    radius: usize,
    x: usize,
    y: usize,
    draw_api: Box<DrawAPI>,
}

impl Circle {
    pub fn new(radius: usize, x: usize, y: usize, draw_api: Box<DrawAPI>) -> Circle {
        Circle { radius, x, y, draw_api }
    }
}

impl Shape for Circle {
    fn draw(&self) {
        self.draw_api.draw_circle(self.radius, self.x, self.y);
    }
}
