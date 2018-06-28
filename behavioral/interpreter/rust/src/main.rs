extern crate interpreter;

use interpreter::*;

fn main() {
    let robert = TerminalExpression::new("Robert".to_string());
    let john = TerminalExpression::new("John".to_string());

    let is_male = OrExpression::new(Box::new(robert), Box::new(john));

    println!("John is male? {}", is_male.interpret(&"John".to_string()));

    let julie = TerminalExpression::new("Julie".to_string());
    let married = TerminalExpression::new("Married".to_string());

    let is_married_woman = AndExpression::new(Box::new(julie), Box::new(married));

    println!("Julie is a married women? {}", is_married_woman.interpret(&"Married Julie".to_string()));
}
