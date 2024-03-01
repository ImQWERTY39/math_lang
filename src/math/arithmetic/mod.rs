const VALID_TOKENS: [char; 15] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '+', '-', '*', '/',
];

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
}

fn tokenize(expression: &str) -> Vec<String> {
    let mut tokenized = Vec::new();

    let mut digit = String::new();
    let mut in_digit = false;

    for i in expression.chars() {
        if i.is_whitespace() {
            continue;
        }

        if !VALID_TOKENS.contains(&i) {
            panic!("Invalid token");
        }

        if i.is_ascii_digit() || i == '.' {
            if !in_digit {
                in_digit = true;
            }

            digit.push(i);
            continue;
        }

        if in_digit {
            tokenized.push(digit.clone());

            digit = String::new();
            in_digit = false;
        }

        tokenized.push(i.to_string());
    }

    if !digit.is_empty() {
        tokenized.push(digit);
    }

    tokenized
}

fn get_lowest_precedence_index(expr: &[&str]) -> usize {
    let mut character = expr.first().unwrap();
    let mut index = 0;

    for (idx, val) in expr.iter().enumerate() {
        if lower_precedence(val, character) {
            character = val;
            index = idx;
        }
    }

    index
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let tokenised1 = tokenize("5 * 2 + 3");
        let tokenised1: Vec<&str> = tokenised1.iter().map(|x| x.as_str()).collect();
        assert_eq!(tokenised1, vec!["5", "*", "2", "+", "3"]);

        let tokenised2 = tokenize("5      *\n 2 + 5.3");
        let tokenised2: Vec<&str> = tokenised2.iter().map(|x| x.as_str()).collect();
        assert_eq!(tokenised2, vec!["5", "*", "2", "+", "5.3"]);
    }
}
