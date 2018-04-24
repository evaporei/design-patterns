extern crate prototype;

use prototype::{CloneFactory, Sheep};

fn main() {
    let animal_maker = CloneFactory {};

    let sheep = Sheep {};

    let cloned_sheep = animal_maker.get_clone(Box::new(sheep));
}
