pub struct Student {
    name: String,
    roll_no: String,
}

impl Student {
    pub fn new() -> Self {
        Student { name: String::new(), roll_no: String::new() }
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn set_roll_no(&mut self, roll_no: &str) {
        self.roll_no = String::from(roll_no);
    }
    pub fn get_roll_no(&self) -> &String {
        &self.roll_no
    }
}

pub struct StudentView;

impl StudentView {
    pub fn print_student_details(&self, name: &str, roll_no: &str) {
        println!("Student: ");
        println!("Name: {}", name);
        println!("Roll No: {}", roll_no);
    }
}

pub struct StudentController {
    model: Student,
    view: StudentView,
}

impl StudentController {
    pub fn new(model: Student, view: StudentView) -> Self {
        StudentController { model, view }
    }
    pub fn set_student_name(&mut self, name: &str) {
        self.model.set_name(name);
    }
    pub fn get_student_name(&self) -> &String {
        self.model.get_name()
    }
    pub fn set_student_roll_no(&mut self, roll_no: &str) {
        self.model.set_roll_no(roll_no);
    }
    pub fn get_student_roll_no(&self) -> &String {
        self.model.get_roll_no()
    }
    pub fn update_view(&self) {
        self.view.print_student_details(self.model.get_name(), self.model.get_roll_no());
    }
}
