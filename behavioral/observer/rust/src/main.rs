extern crate observer;

use observer::*;

fn main() {
    let mut subject = Subject::new();

    subject.attach(Box::new(BinaryObserver));
    subject.attach(Box::new(OctalObserver));
    subject.attach(Box::new(HexaObserver));

    println!("First state change: 15");
    subject.set_state(15);

    println!("Second state change: 10");
    subject.set_state(10);
}
