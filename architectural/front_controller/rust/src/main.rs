extern crate front_controller;

use front_controller::FrontController;

fn main() {
    let controller = FrontController::new();

    controller.dispatch_request("HOME");
    controller.dispatch_request("STUDENT");
}
