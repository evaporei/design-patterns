#[derive(Clone, PartialEq)]
pub struct Person {
    name: String,
    gender: String,
    marital_status: String,
}

impl Person {
    pub fn new(name: String, gender: String, marital_status: String) -> Self {
        Person { name, gender, marital_status }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_gender(&self) -> &String {
        &self.gender
    }
    pub fn get_marital_status(&self) -> &String {
        &self.marital_status
    }
}

pub trait Criteria {
    fn meet_criteria(&self, persons: &Vec<Person>) -> Vec<Person>;
}

pub struct CriteriaMale;

impl Criteria for CriteriaMale {
    fn meet_criteria(&self, persons: &Vec<Person>) -> Vec<Person> {
        let mut filtered_persons = Vec::new();
        for person in persons {
            if person.get_gender().to_lowercase() == "male" {
                filtered_persons.push(person.clone());
            }
        }
        filtered_persons
    }
}

pub struct CriteriaFemale;

impl Criteria for CriteriaFemale {
    fn meet_criteria(&self, persons: &Vec<Person>) -> Vec<Person> {
        let mut filtered_persons = Vec::new();
        for person in persons {
            if person.get_gender().to_lowercase() == "female" {
                filtered_persons.push(person.clone());
            }
        }
        filtered_persons
    }
}

pub struct CriteriaSingle;

impl Criteria for CriteriaSingle {
    fn meet_criteria(&self, persons: &Vec<Person>) -> Vec<Person> {
        let mut filtered_persons = Vec::new();
        for person in persons {
            if person.get_marital_status().to_lowercase() == "single" {
                filtered_persons.push(person.clone());
            }
        }
        filtered_persons
    }
}

pub struct AndCriteria {
    criteria: Box<Criteria>,
    other_criteria: Box<Criteria>,
}

impl AndCriteria {
    pub fn new(criteria: Box<Criteria>, other_criteria: Box<Criteria>) -> Self {
        AndCriteria { criteria, other_criteria }
    }
}

impl Criteria for AndCriteria {
    fn meet_criteria(&self, persons: &Vec<Person>) -> Vec<Person> {
        let first_criteria_persons = self.criteria.meet_criteria(persons);

        self.other_criteria.meet_criteria(&first_criteria_persons)
    }
}

pub struct OrCriteria {
    criteria: Box<Criteria>,
    other_criteria: Box<Criteria>,
}

impl OrCriteria {
    pub fn new(criteria: Box<Criteria>, other_criteria: Box<Criteria>) -> Self {
        OrCriteria { criteria, other_criteria }
    }
}

impl Criteria for OrCriteria {
    fn meet_criteria(&self, persons: &Vec<Person>) -> Vec<Person> {
        let mut first_criteria_persons = self.criteria.meet_criteria(persons);
        let other_criteria_persons = self.other_criteria.meet_criteria(persons);

        for person in other_criteria_persons {
            if !first_criteria_persons.contains(&person) {
                first_criteria_persons.push(person);
            }
        }

        first_criteria_persons
    }
}
