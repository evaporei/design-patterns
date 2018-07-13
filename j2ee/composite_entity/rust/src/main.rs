extern crate composite_entity;

use composite_entity::Client;

fn main() {
    let mut client = Client::new();

    client.set_data("Test", "Data");
    client.print_data();

    client.set_data("Second Test", "Data1");
    client.print_data();
}
