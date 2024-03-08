use super::{
    parser,
    tokenizer::{self, Tokens},
};
use crate::Scope;

type InbuiltFunction = dyn Fn(Vec<Expression>, &Scope) -> f64;

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
    Function(Box<InbuiltFunction>),
}

pub enum EvaluationResult {
    Number(f64),
    AssignNumber(String, f64),
}

impl EvaluationResult {
    fn get_number(&self) -> f64 {
        if let Self::Number(i) = self {
            return *i;
        }

        panic!("Not a number");
    }
}

impl From<f64> for EvaluationResult {
    fn from(value: f64) -> Self {
        EvaluationResult::Number(value)
    }
}

impl Expression {
    pub fn evaluate(&self, scope: &Scope) -> EvaluationResult {
        match self {
            Expression::Number(i) => (*i).into(),
            Expression::Variable(i) => scope.get(i).unwrap().evaluate(scope),
            Expression::Negate(i) => {
                EvaluationResult::Number(i.evaluate(scope).get_number() * -1.0)
            }
            Expression::Add(i, j) => {
                (i.evaluate(scope).get_number() + j.evaluate(scope).get_number()).into()
            }
            Expression::Subtract(i, j) => {
                (i.evaluate(scope).get_number() - j.evaluate(scope).get_number()).into()
            }
            Expression::Multiply(i, j) => {
                (i.evaluate(scope).get_number() * j.evaluate(scope).get_number()).into()
            }
            Expression::Divide(i, j) => {
                (i.evaluate(scope).get_number() / j.evaluate(scope).get_number()).into()
            }
            Expression::Power(i, j) => i
                .evaluate(scope)
                .get_number()
                .powf(j.evaluate(scope).get_number())
                .into(),
            Expression::Equate(i, j) => {
                if let Expression::Variable(ref var) = **i {
                    return EvaluationResult::AssignNumber(
                        var.clone(),
                        j.evaluate(scope).get_number(),
                    );
                }

                unimplemented!()
            }
            _ => unreachable!(),
        }
    }

    pub fn parse(expression: &str, scope: &Scope) -> Option<Self> {
        let expr = tokenizer::tokenize(expression, scope)?;

        let equate_count = expr.iter().filter(|x| matches!(x, Tokens::Equate)).count();

        if equate_count > 1 {
            return None;
        }

        parser::parse(&expr, scope)
    }
}
