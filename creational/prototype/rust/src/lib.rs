pub trait Animal {
    fn make_clone(&self) -> Box<Animal>;
}

#[derive(Clone)]
pub struct Sheep {}

impl Animal for Sheep {
    fn make_clone(&self) -> Box<Animal> {
        Box::new(self.clone())
    }
}

pub struct CloneFactory {}

impl CloneFactory {
    pub fn get_clone(&self, animal: Box<Animal>) -> Box<Animal> {
        animal.make_clone()
    }
}
