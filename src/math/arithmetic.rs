use super::{parser, tokenizer};

#[derive(Debug)]
pub enum ArithmeticExpression {
    Number(f64),
    Add(Box<Self>, Box<Self>),
    Subtract(Box<Self>, Box<Self>),
    Multiply(Box<Self>, Box<Self>),
    Divide(Box<Self>, Box<Self>),
}

impl ArithmeticExpression {
    pub fn evaluate(&self) -> f64 {
        match self {
            Self::Number(i) => *i,
            Self::Add(i, j) => i.evaluate() + j.evaluate(),
            Self::Subtract(i, j) => i.evaluate() - j.evaluate(),
            Self::Multiply(i, j) => i.evaluate() * j.evaluate(),
            Self::Divide(i, j) => i.evaluate() / j.evaluate(),
        }
    }

    pub fn parse(expression: &str) -> Option<Self> {
        let expr = tokenizer::tokenize(expression);
        let expr_tokens: Vec<&str> = expr.iter().map(|x| x.as_str()).collect();

        parser::parse(&expr_tokens)
    }
}
