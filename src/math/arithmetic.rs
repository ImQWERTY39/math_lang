use super::{parser, tokenizer};
use crate::Scope;

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
    Function(Box<dyn Fn(Vec<Expression>, &Scope) -> f64>),
}

/*
float pi = 3.14159;

float f(x) {
    x^2 + 10x - 9pi
}

1. UserFunction(Box<Expression>, scope)
2. UserFunction(vec<args>, expression)

for i in args {
    expression = expression.replace(i, {});
}

Expression::parse(expression)

f(x) => scope [Expression::Variable("x")]
*/

pub enum EvaluationResult {}

impl Expression {
    pub fn evaluate(&self, scope: &Scope) -> f64 {
        match self {
            Expression::Number(i) => *i,
            Expression::Variable(i) => scope.get(i).unwrap().evaluate(scope),
            Expression::Negate(i) => i.evaluate(scope) * -1.0,
            Expression::Add(i, j) => i.evaluate(scope) + j.evaluate(scope),
            Expression::Subtract(i, j) => i.evaluate(scope) - j.evaluate(scope),
            Expression::Multiply(i, j) => i.evaluate(scope) * j.evaluate(scope),
            Expression::Divide(i, j) => i.evaluate(scope) / j.evaluate(scope),
            Expression::Power(i, j) => i.evaluate(scope).powf(j.evaluate(scope)),
            _ => unimplemented!(),
        }
    }

    pub fn parse(expression: &str, scope: &Scope) -> Option<Self> {
        let expr = tokenizer::tokenize(expression, scope)?;
        parser::parse(&expr, scope)
    }
}
