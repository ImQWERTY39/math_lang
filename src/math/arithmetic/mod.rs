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

    pub fn parse(expression: &str) -> Self {
        let expr = tokenize(expression);

        if expr.is_empty() {
            panic!("Emptry expression");
        }

        // match get_lowest_precedence(&expr) {
        //     "+" => Self::Add(Box::new()),
        // }
    }
}

fn tokenize(expression: &str) -> Vec<&str> {
    let mut tokenized = Vec::new();

    for i in expression.chars() {
        if i.is_digit(10) {}
    }

    tokenized
}

// fn get_lowest_precedence<'a>(expr: &'a [&str]) -> &'a str {}
