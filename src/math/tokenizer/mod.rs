use crate::{Expression, Scope};

#[derive(Debug, Clone)]
pub enum Tokens {
    Number(f64),
    Variable(String),
    Add,
    Subtract,
    Negate,
    Multiply,
    Divide,
    Power,
    Comma,
    Equate,
    OpenBrackets,
    CloseBrackets,
    Function(String, Vec<Vec<Tokens>>),
}

impl TryFrom<char> for Tokens {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '+' => Tokens::Add,
            '-' => Tokens::Subtract,
            '*' => Tokens::Multiply,
            '/' => Tokens::Divide,
            '^' => Tokens::Power,
            '=' => Tokens::Equate,
            '(' => Tokens::OpenBrackets,
            ')' => Tokens::CloseBrackets,
            ',' => Tokens::Comma,
            _ => return Err(()),
        })
    }
}

pub fn tokenize(expression: &str, scope: &Scope) -> Option<Vec<Tokens>> {
    let mut tokens = Vec::new();

    let mut digit = String::new();
    let mut in_digit = false;

    let mut word = String::new();
    let mut in_word = false;

    for i in expression.chars().filter(|x| !x.is_whitespace()) {
        if i.is_ascii_alphabetic() {
            if in_digit {
                tokens.push(Tokens::Number(digit.parse().unwrap()));
                tokens.push(Tokens::Multiply);

                digit.clear();
                in_digit = false;
            }

            if !in_word {
                in_word = true;
            }

            word.push(i);
            continue;
        }

        if in_word {
            tokens.push(Tokens::Variable(word.clone()));

            word.clear();
            in_word = false;
        }

        if i.is_ascii_digit() || i == '.' {
            if !in_digit {
                in_digit = true;
            }

            digit.push(i);
            continue;
        }

        if in_digit {
            tokens.push(Tokens::Number(digit.parse().unwrap()));

            digit.clear();
            in_digit = false;
        }

        tokens.push(i.try_into().ok()?);
    }

    if in_digit {
        tokens.push(Tokens::Number(digit.parse().unwrap()));
    }

    if in_word {
        tokens.push(Tokens::Variable(word));
    }

    correct(tokens, scope)
}

fn correct(tokens: Vec<Tokens>, scope: &Scope) -> Option<Vec<Tokens>> {
    let mut tokens_corrected = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        if let Tokens::Subtract = tokens[i] {
            if i == 0 {
                tokens_corrected.push(Tokens::Negate);
            } else {
                match tokens[i - 1] {
                    Tokens::Number(_) | Tokens::Variable(_) | Tokens::Function(_, _) => {
                        tokens_corrected.push(Tokens::Subtract)
                    }
                    _ => {
                        tokens_corrected.push(Tokens::Negate);
                    }
                }
            }
        } else if let Tokens::OpenBrackets = tokens[i] {
            if i == 0 {
                tokens_corrected.push(Tokens::OpenBrackets);
                i += 1;
                continue;
            }

            if matches!(
                tokens.get(i - 1).unwrap(),
                Tokens::CloseBrackets | Tokens::Number(_) | Tokens::Variable(_)
            ) {
                tokens_corrected.push(Tokens::Multiply);
                tokens_corrected.push(tokens[i].clone());
            }
            tokens_corrected.push(Tokens::OpenBrackets);
        } else if let Tokens::Variable(ref var_name) = tokens[i] {
            /*
            check if its a defined function
            or = after function name
            */
            let in_built = scope
                .get(var_name)
                .is_some_and(|x| matches!(x, Expression::Function(..)));

            let user_defined = match scope.get(var_name) {
                Some(i) => match i {
                    Expression::Number(_) | Expression::Variable(_) => false,
                    _ => true,
                },
                None => matches!(tokens[i + 1], Tokens::OpenBrackets),
            };

            if in_built {
                i += 2;

                let mut arguments = Vec::new();
                let mut current = Vec::new();

                let mut open_brac = 1;

                while i < tokens.len() && open_brac > 0 {
                    let token = tokens[i].clone();

                    match token {
                        Tokens::Comma if open_brac == 1 => {
                            arguments.push(correct(current.clone(), scope)?);
                            current.clear();
                        }
                        Tokens::OpenBrackets => {
                            open_brac += 1;

                            if open_brac > 1 {
                                current.push(Tokens::OpenBrackets);
                            }
                        }
                        Tokens::CloseBrackets => {
                            open_brac -= 1;

                            if open_brac > 0 {
                                current.push(Tokens::CloseBrackets);
                            }
                        }
                        other => current.push(other),
                    }

                    i += 1;
                }

                if !current.is_empty() {
                    arguments.push(correct(current, scope)?);
                }

                tokens_corrected.push(Tokens::Function(var_name.to_owned(), arguments));
                continue;
            } else if user_defined {
                unimplemented!("{:?} | {} | {}", tokens[i], in_built, user_defined);
            } else {
                tokens_corrected.push(tokens[i].clone());
            }
        } else {
            tokens_corrected.push(tokens[i].clone());
        }

        i += 1;
    }

    Some(tokens_corrected)
}
