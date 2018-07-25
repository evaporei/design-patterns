extern crate data_transfer_object;

use data_transfer_object::{NameInterface, NumberInterface, DataTransferObject};

fn main() {
    let mut data_transfer_object = DataTransferObject::new();

    data_transfer_object.set_name("Otávio");
    data_transfer_object.set_number(15);

    println!("Customer name: {}", data_transfer_object.get_name());
    println!("Address number: {}", data_transfer_object.get_number());
}
