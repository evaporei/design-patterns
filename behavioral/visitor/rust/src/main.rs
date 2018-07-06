extern crate visitor;

use visitor::*;

fn main() {
    let computer = ComputerPart::Computer {
        parts: vec![ComputerPart::Keyboard, ComputerPart::Monitor, ComputerPart::Mouse],
    };
    computer.accept(&ComputerPartDisplayVisitor);
}
