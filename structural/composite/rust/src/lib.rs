pub struct Employee {
    name: String,
    dept: String,
    salary: usize,
    subordinates: Vec<Employee>,
}

impl Employee {
    pub fn new(name: String, dept: String, salary: usize) -> Self {
        Employee {
            name,
            dept,
            salary,
            subordinates: vec![],
        }
    }
    pub fn add(&mut self, employee: Employee) {
        self.subordinates.push(employee);
    }
    pub fn get_subordinates(&self) -> &Vec<Employee> {
        &self.subordinates
    }
}

use std::fmt;

impl fmt::Display for Employee {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Employee: [ Name: {}, Dept: {}, Salary: {} ]", self.name, self.dept, self.salary)
    }
}
