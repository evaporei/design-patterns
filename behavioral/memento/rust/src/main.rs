extern crate memento;

use memento::*;

fn main() {
    let mut originator = Originator::new("State #1");
    let mut care_taker = CareTaker::new();

    originator.set_state("State #2");
    care_taker.add(originator.save_state_to_memento());

    originator.set_state("State #3");
    care_taker.add(originator.save_state_to_memento());

    originator.set_state("State #4");
    println!("Current State: {}", originator.get_state());

    originator.get_state_from_memento(care_taker.get(0));
    println!("First saved State: {}", originator.get_state());

    originator.get_state_from_memento(care_taker.get(1));
    println!("Second saved State: {}", originator.get_state());
}
