extern crate data_access_object;

use data_access_object::*;

fn main() {
    let mut student_dao = StudentDAOImpl::new();

    for student in student_dao.get_all_students() {
        println!("Student: [RollNo : {}, Name : {} ]", student.get_roll_no(), student.get_name());
    }

    match student_dao.get_student(0) {
        Ok(mut student) => {
            println!("Student: [RollNo : {}, Name : {} ]", student.get_roll_no(), student.get_name());

            student.set_name("Michael");
            match student_dao.update_student(&student) {
                Ok(()) => println!("Student: [RollNo : {}, Name : {} ]", student.get_roll_no(), student.get_name()),
                Err(error) => println!("update error: {}", error),
            }
        },
        Err(error) => println!("get error: {}", error)
    }
}
