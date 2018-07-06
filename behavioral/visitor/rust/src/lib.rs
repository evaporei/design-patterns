pub enum ComputerPart {
    Keyboard,
    Monitor,
    Mouse,
    Computer { parts: Vec<ComputerPart> },
}

impl ComputerPart {
    pub fn accept(&self, computer_part_visitor: &ComputerPartVisitor) {
        match *self {
            ComputerPart::Keyboard | ComputerPart::Monitor | ComputerPart::Mouse =>
                computer_part_visitor.visit(&self),
            ComputerPart::Computer { ref parts } => {
                for part in parts.iter() {
                    part.accept(computer_part_visitor);
                }
                computer_part_visitor.visit(&self);
            },
        };
    }
}

pub trait ComputerPartVisitor {
    fn visit(&self, computer_part: &ComputerPart);
}

pub struct ComputerPartDisplayVisitor;

impl ComputerPartVisitor for ComputerPartDisplayVisitor {
    fn visit(&self, computer_part: &ComputerPart) {
        match *computer_part {
            ComputerPart::Keyboard => println!("Displaying Keyboard."),
            ComputerPart::Monitor => println!("Displaying Monitor."),
            ComputerPart::Mouse => println!("Displaying Mouse."),
            ComputerPart::Computer { ref parts } => println!("Displaying Computer."),
        };
    }
}
