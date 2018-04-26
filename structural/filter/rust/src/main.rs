extern crate filter;

use filter::Person;
use filter::Criteria;
use filter::CriteriaMale;
use filter::CriteriaFemale;
use filter::CriteriaSingle;
use filter::AndCriteria;
use filter::OrCriteria;

fn main() {
    let mut persons = Vec::new();

    persons.push(Person::new("Robert".to_string(), "Male".to_string(), "Single".to_string()));
    persons.push(Person::new("John".to_string(), "Male".to_string(), "Married".to_string()));
    persons.push(Person::new("Laura".to_string(), "Female".to_string(), "Married".to_string()));
    persons.push(Person::new("Diana".to_string(), "Female".to_string(), "Single".to_string()));
    persons.push(Person::new("Mike".to_string(), "Male".to_string(), "Single".to_string()));
    persons.push(Person::new("Bobby".to_string(), "Male".to_string(), "Single".to_string()));

    let male = CriteriaMale;
    let female = CriteriaFemale;
    let single_male = AndCriteria::new(Box::new(CriteriaMale), Box::new(CriteriaSingle));
    let single_or_female = OrCriteria::new(Box::new(CriteriaFemale), Box::new(CriteriaSingle));

    println!("Males: ");
    print_persons(&male.meet_criteria(&persons));

    println!("\nFemales: ");
    print_persons(&female.meet_criteria(&persons));

    println!("\nSingle Males: ");
    print_persons(&single_male.meet_criteria(&persons));

    println!("\nSingle Or Females: ");
    print_persons(&single_or_female.meet_criteria(&persons));
}

fn print_persons(persons: &Vec<Person>) {
    for person in persons {
        println!("Person: [ Name: {}, Gender: {}, Marital Status: {} ]", person.get_name(), person.get_gender(), person.get_marital_status());
    }
}
