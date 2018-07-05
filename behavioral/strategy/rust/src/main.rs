extern crate strategy;

use strategy::*;

fn main() {
    let mut context = Context::new(Box::new(OperationAdd));
    println!("10 + 5 = {}", context.execute_strategy(10, 5));

    context = Context::new(Box::new(OperationSubtract));
    println!("10 - 5 = {}", context.execute_strategy(10, 5));

    context = Context::new(Box::new(OperationMultiply));
    println!("10 * 5 = {}", context.execute_strategy(10, 5));
}
