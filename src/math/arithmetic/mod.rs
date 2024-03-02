mod parser;
mod tokenizer;

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
        let expr = tokenizer::tokenize(expression);
        let expr_tokens: Vec<&str> = expr.iter().map(|x| x.as_str()).collect();

        parser::parse(&expr_tokens)
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use parser::*;
//     use tokenizer::*;

//     fn compare(tokenized: Vec<String>, expected: Vec<&str>) -> bool {
//         tokenized.iter().map(|x| x.as_str()).collect::<Vec<&str>>() == expected
//     }

//     #[test]
//     fn tokenizing_normal() {
//         assert!(compare(tokenize("2 + 3"), vec!["2", "+", "3"]));
//         assert!(compare(tokenize("2 - 3"), vec!["2", "-", "3"]));
//         assert!(compare(tokenize("2 * 3"), vec!["2", "*", "3"]));
//         assert!(compare(tokenize("2 / 3"), vec!["2", "/", "3"]));
//         assert!(compare(tokenize("2213 + 433"), vec!["2213", "+", "433"]));
//         assert!(compare(
//             tokenize("2 2 1 3 + 433  "),
//             vec!["2", "2", "1", "3", "+", "433"]
//         ));
//         assert!(compare(
//             tokenize("2213.234 + 433 - 0"),
//             vec!["2213.234", "+", "433", "-", "0"]
//         ));
//     }

//     #[test]
//     fn tokenizing_with_whitespace() {
//         let tokenized1 = tokenize("5   *\n  2  \t   +    3");
//         assert!(compare(tokenized1, vec!["5", "*", "2", "+", "3"]))
//     }

//     #[test]
//     fn lowest_precedence_check() {
//         assert_eq!(get_lowest_precedence_index(&["2", "+", "3"]), 1);
//         assert_eq!(get_lowest_precedence_index(&["2", "3"]), 0);
//         assert_eq!(get_lowest_precedence_index(&["2", "*", "3", "+", "5.6"]), 3);
//     }
// }
