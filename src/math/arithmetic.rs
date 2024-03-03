use std::collections::HashMap;

use super::{parser, tokenizer};

#[derive(Debug)]
pub enum ArithmeticExpression {
    Number(f64),
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
    pub fn evaluate(&self) -> EvaluationResult {
        match self {
            Self::Number(i) => EvaluationResult::Number(*i),
            Self::Add(i, j) => {
                EvaluationResult::Number(i.evaluate().get_number() + j.evaluate().get_number())
            }
            Self::Subtract(i, j) => {
                EvaluationResult::Number(i.evaluate().get_number() - j.evaluate().get_number())
            }
            Self::Multiply(i, j) => {
                EvaluationResult::Number(i.evaluate().get_number() * j.evaluate().get_number())
            }
            Self::Divide(i, j) => {
                EvaluationResult::Number(i.evaluate().get_number() / j.evaluate().get_number())
            }
            Self::Power(i, j) => {
                EvaluationResult::Number(i.evaluate().get_number().powf(j.evaluate().get_number()))
            }
        }
    }

    pub fn parse(expression: &str, scope: &HashMap<String, Self>) -> Option<Self> {
        let expr: Vec<String> = tokenizer::tokenize(expression)
            .iter()
            .map(|x| {
                if scope.contains_key(x) {
                    if let Self::Number(i) = scope.get(x).unwrap() {
                        return i.to_string();
                    }
                }

                x.to_owned()
            })
            .collect();
        let expr_tokens: Vec<&str> = expr.iter().map(|x| x.as_str()).collect();

        parser::parse(&expr_tokens)
    }
}
