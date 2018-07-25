extern crate service_locator;

use service_locator::ServiceLocator;

fn main() {
    let mut service_locator = ServiceLocator::new();

    if let Some(service) = service_locator.get_service("Service1") {
        service.execute();
    }

    if let Some(service) = service_locator.get_service("Service2") {
        service.execute();
    }

    if let Some(service) = service_locator.get_service("Service1") {
        service.execute();
    }

    if let Some(service) = service_locator.get_service("Service2") {
        service.execute();
    }
}
