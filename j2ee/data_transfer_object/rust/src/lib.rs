struct Customer {
    name: String,
}

pub trait NameInterface {
    fn set_name(&mut self, name: &str);
    fn get_name(&self) -> &String;
}

impl Customer {
    fn new() -> Self {
        Customer { name: String::new() }
    }
}

impl NameInterface for Customer {
    fn set_name(&mut self, name: &str) {
        self.name = String::from(name)
    }
    fn get_name(&self) -> &String {
        &self.name
    }
}

struct Address {
    number: usize,
}

pub trait NumberInterface {
    fn set_number(&mut self, number: usize);
    fn get_number(&self) -> usize;
}

impl Address {
    fn new() -> Self {
        Address { number: 0 }
    }
}

impl NumberInterface for Address {
    fn set_number(&mut self, number: usize) {
        self.number = number;
    }
    fn get_number(&self) -> usize {
        self.number
    }
}

pub struct DataTransferObject {
    customer_name: String,
    address_number: usize,
}

impl DataTransferObject {
    pub fn new() -> Self {
        DataTransferObject { customer_name: String::new(), address_number: 0 }
    }
}

impl NameInterface for DataTransferObject {
    fn set_name(&mut self, name: &str) {
        self.customer_name = String::from(name);
    }
    fn get_name(&self) -> &String {
        &self.customer_name
    }
}

impl NumberInterface for DataTransferObject {
    fn set_number(&mut self, number: usize) {
        self.address_number = number;
    }
    fn get_number(&self) -> usize {
        self.address_number
    }
}
