extern crate iterator;

use iterator::*;

fn main() {
    let mut names_repository = NameRepository::new();

    while let Some(name) = names_repository.iterator.next() {
        println!("Name : {}", name);
    }
}
