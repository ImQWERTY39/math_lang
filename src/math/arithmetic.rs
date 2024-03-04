use std::collections::HashMap;

use super::{parser, tokenizer};

#[derive(Debug)]
pub enum ArithmeticExpression {
    Number(f64),
    Vairable(String),
    Add(Box<Self>, Box<Self>),
    Subtract(Box<Self>, Box<Self>),
    Multiply(Box<Self>, Box<Self>),
    Divide(Box<Self>, Box<Self>),
    Power(Box<Self>, Box<Self>),
}

#[derive(Debug)]
pub enum EvaluationResult {
    Number(f64),
    Equality(bool),
    Assignment(String, ArithmeticExpression),
}

impl EvaluationResult {
    fn get_number(&self) -> f64 {
        if let Self::Number(i) = self {
            return *i;
        }

        panic!("Evaluation result is not a number");
    }
}

impl ArithmeticExpression {
    pub fn evaluate(&self, scope: &HashMap<String, Self>) -> EvaluationResult {
        match self {
            Self::Number(i) => EvaluationResult::Number(*i),
            Self::Vairable(i) => {
                if let Some(i) = scope.get(i) {
                    if let Self::Number(j) = i {
                        return EvaluationResult::Number(*j);
                    }
                }

                panic!("Variable doesn't exist");
            }
            Self::Add(i, j) => EvaluationResult::Number(
                i.evaluate(scope).get_number() + j.evaluate(scope).get_number(),
            ),
            Self::Subtract(i, j) => EvaluationResult::Number(
                i.evaluate(scope).get_number() - j.evaluate(scope).get_number(),
            ),
            Self::Multiply(i, j) => EvaluationResult::Number(
                i.evaluate(scope).get_number() * j.evaluate(scope).get_number(),
            ),
            Self::Divide(i, j) => EvaluationResult::Number(
                i.evaluate(scope).get_number() / j.evaluate(scope).get_number(),
            ),
            Self::Power(i, j) => EvaluationResult::Number(
                i.evaluate(scope)
                    .get_number()
                    .powf(j.evaluate(scope).get_number()),
            ),
        }
    }

    pub fn parse(expression: &str) -> Option<Self> {
        let expr = tokenizer::tokenize(expression)?;
        let expr_tokens: Vec<&str> = expr.iter().map(|x| x.as_str()).collect();

        parser::parse(&expr_tokens)
    }
}
