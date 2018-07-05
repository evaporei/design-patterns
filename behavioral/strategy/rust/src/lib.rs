pub trait Strategy {
    fn do_operation(&self, number1: isize, number2: isize) -> isize;
}

pub struct OperationAdd;

impl Strategy for OperationAdd {
    fn do_operation(&self, number1: isize, number2: isize) -> isize {
        number1 + number2
    }
}

pub struct OperationSubtract;

impl Strategy for OperationSubtract {
    fn do_operation(&self, number1: isize, number2: isize) -> isize {
        number1 - number2
    }
}

pub struct OperationMultiply;

impl Strategy for OperationMultiply {
    fn do_operation(&self, number1: isize, number2: isize) -> isize {
        number1 * number2
    }
}

pub struct Context {
    strategy: Box<Strategy>,
}

impl Context {
    pub fn new(strategy: Box<Strategy>) -> Self {
        Context { strategy }
    }
    pub fn execute_strategy(&self, number1: isize, number2: isize) -> isize {
        self.strategy.do_operation(number1, number2)
    }
}
