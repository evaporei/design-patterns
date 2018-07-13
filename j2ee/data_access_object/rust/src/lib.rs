#[derive(Clone)]
pub struct Student {
    name: String,
    roll_no: usize,
}

impl Student {
    fn new(name: &str, roll_no: usize) -> Self {
        Student { name: String::from(name), roll_no }
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn set_roll_no(&mut self, roll_no: usize) {
        self.roll_no = roll_no;
    }
    pub fn get_roll_no(&self) -> usize {
        self.roll_no
    }
}

pub trait StudentDAO {
    fn get_all_students(&self) -> &Vec<Student>;
    fn get_student(&self, roll_no: usize) -> Result<Student, DatabaseNotFoundError>;
    fn update_student(&mut self, student: &Student) -> Result<(), DatabaseNotFoundError>;
    fn delete_student(&mut self, roll_no: usize) -> Result<(), DatabaseNotFoundError>;
}

pub struct StudentDAOImpl {
    students: Vec<Student>,
}

impl StudentDAOImpl {
    pub fn new() -> Self {
        StudentDAOImpl { students: fake_students_database() }
    }
}

fn fake_students_database() -> Vec<Student> {
    vec![
        Student::new("Robert", 0),
        Student::new("John", 1),
    ]
}

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DatabaseNotFoundError;

impl Error for DatabaseNotFoundError {
    fn description(&self) -> &str {
        "There is no record found for these parameters on the database"
    }
}

impl fmt::Display for DatabaseNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DatabaseNotFoundError: there is no record found for these parameters")
    }
}

impl StudentDAO for StudentDAOImpl {
    fn get_all_students(&self) -> &Vec<Student> {
        &self.students
    }
    fn get_student(&self, roll_no: usize) -> Result<Student, DatabaseNotFoundError> {
        match self.students.get(roll_no) {
            Some(student) => Ok(student.clone()),
            None => Err(DatabaseNotFoundError),
        }
    }
    fn update_student(&mut self, student: &Student) -> Result<(), DatabaseNotFoundError> {
        match self.get_student(student.get_roll_no()) {
            Ok(mut clone_of_database_student) => {
                clone_of_database_student.set_name(student.get_name());
                self.students[student.get_roll_no()] = clone_of_database_student;
                Ok(())
            },
            Err(error) => Err(error),
        }
    }
    fn delete_student(&mut self, roll_no: usize) -> Result<(), DatabaseNotFoundError> {
        match self.get_student(roll_no) {
            Ok(_database_student) => {
                self.students.remove(roll_no);
                Ok(())
            },
            Err(error) => Err(error),
        }
    }
}
