extern crate composite;

use composite::Employee;

fn main() {
    let mut ceo = Employee::new("Maria".to_string(), "CEO".to_string(), 30000);

    let mut head_sales = Employee::new("Robert".to_string(), "Head Sales".to_string(), 20000);

    let mut head_marketing = Employee::new("José".to_string(), "Head Marketing".to_string(), 20000);

    let clerk1 = Employee::new("Laura".to_string(), "Marketing".to_string(), 10000);
    let clerk2 = Employee::new("John".to_string(), "Marketing".to_string(), 10000);

    let sales_executive1 = Employee::new("Richard".to_string(), "Sales".to_string(), 10000);
    let sales_executive2 = Employee::new("Samantha".to_string(), "Sales".to_string(), 10000);

    head_marketing.add(clerk1);
    head_marketing.add(clerk2);

    head_sales.add(sales_executive1);
    head_sales.add(sales_executive2);

    ceo.add(head_sales);
    ceo.add(head_marketing);

    println!("{}", ceo);

    for head_employee in ceo.get_subordinates() {
        println!("{}", head_employee);

        for employee in head_employee.get_subordinates() {
            println!("{}", employee);
        }
    }
}
