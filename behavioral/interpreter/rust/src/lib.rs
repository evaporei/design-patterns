pub trait Expression {
    fn interpret(&self, context: &String) -> bool;
}

pub struct TerminalExpression {
    data: String,
}

impl TerminalExpression {
    pub fn new(data: String) -> Self {
        TerminalExpression { data }
    }
}

impl Expression for TerminalExpression {
    fn interpret(&self, context: &String) -> bool {
        if context.contains(&self.data) {
            return true
        }
        false
    }
}

pub struct OrExpression {
    expression1: Box<Expression>,
    expression2: Box<Expression>,
}

impl OrExpression {
    pub fn new(expression1: Box<Expression>, expression2: Box<Expression>) -> Self {
        OrExpression { expression1, expression2 }
    }
}

impl Expression for OrExpression {
    fn interpret(&self, context: &String) -> bool {
        self.expression1.interpret(context) || self.expression2.interpret(context)
    }
}

pub struct AndExpression {
    expression1: Box<Expression>,
    expression2: Box<Expression>,
}

impl AndExpression {
    pub fn new(expression1: Box<Expression>, expression2: Box<Expression>) -> Self {
        AndExpression { expression1, expression2 }
    }
}

impl Expression for AndExpression {
    fn interpret(&self, context: &String) -> bool {
        self.expression1.interpret(context) && self.expression2.interpret(context)
    }
}
