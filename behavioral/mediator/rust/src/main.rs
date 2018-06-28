extern crate mediator;

use mediator::*;

fn main() {
    let robert = User::new("Robert");
    let john = User::new("John");

    robert.send_message("Hi, John!");
    john.send_message("Hello! Robert!");
}
