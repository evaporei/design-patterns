extern crate business_delegate;

use business_delegate::*;

fn main() {
    let mut business_delegate = BusinessDelegate::new();
    business_delegate.set_service_type("EJB");

    let mut client = Client::new(&mut business_delegate);
    client.do_task();

    client.business_service.set_service_type("JMS");
    client.do_task();
}
