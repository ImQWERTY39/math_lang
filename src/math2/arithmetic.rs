pub enum Expression {
    Number(f64),
    Variable(String),
    Negate(Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Power(Box<Expression>, Box<Expression>),
    Equate(Box<Expression>, Box<Expression>),
    // Function(fn(Vec<Expression>) -> f64),
    Function(i32),
}
