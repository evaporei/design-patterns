extern crate null_object;

use null_object::*;

fn main() {
    let customer_factory = CustomerFactory::new(vec!["Rob", "Joe", "Julie"]);

    let customer1 = customer_factory.get_customer("Rob");
    let customer2 = customer_factory.get_customer("Bob");
    let customer3 = customer_factory.get_customer("Julie");
    let customer4 = customer_factory.get_customer("Laura");

    println!("Customers");

    println!("{}", customer1.get_name());
    println!("{}", customer2.get_name());
    println!("{}", customer3.get_name());
    println!("{}", customer4.get_name());
}
