pub trait Color {
    fn fill(&self);
}

pub struct Red {}

impl Color for Red {
    fn fill(&self) {
        println!("Inside Red::fill method.");
    }
}

pub struct Green {}

impl Color for Green {
    fn fill(&self) {
        println!("Inside Green::fill method.");
    }
}

pub struct Blue {}

impl Color for Blue {
    fn fill(&self) {
        println!("Inside Blue::fill method.");
    }
}

