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

impl ArithmeticExpression {
    pub fn evaluate(&self) -> f64 {
        match self {
            Self::Number(i) => *i,
            Self::Add(i, j) => i.evaluate() + j.evaluate(),
            Self::Subtract(i, j) => i.evaluate() - j.evaluate(),
            Self::Multiply(i, j) => i.evaluate() * j.evaluate(),
            Self::Divide(i, j) => i.evaluate() / j.evaluate(),
            Self::Power(i, j) => i.evaluate().powf(j.evaluate()),
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
