extern crate mvc;

use mvc::*;

fn main() {
    let mut model = Student::new();

    model.set_name("Robert");
    model.set_roll_no("10");

    let view = StudentView;

    let mut controller = StudentController::new(model, view);

    controller.update_view();
    controller.set_student_name("John");
    controller.update_view();
}
