use super::{tokenizer::Tokens, Expression};
use crate::Scope;

pub fn parse(expr: &[Tokens], scope: &Scope) -> Option<Expression> {
    if expr.is_empty() {
        return None;
    }

    if expr.len() == 1 {
        return match expr.first().unwrap() {
            Tokens::Number(i) => Some(Expression::Number(*i)),
            Tokens::Variable(i) => Some(Expression::Variable(i.clone())),
            Tokens::Function(name, args) => {
                Some(Expression::Number(call_function(scope, name, args)))
            }
            _ => None,
        };
    }

    if matches!(expr.first().unwrap(), Tokens::Negate) {
        return Some(Expression::Negate(Box::new(parse(&expr[1..], scope)?)));
    }

    if entire_expr_in_paren(expr) {
        return parse(&expr[1..expr.len() - 1], scope);
    }

    let idx = get_lowest_precedence(expr);
    Some(match expr[idx] {
        Tokens::Equate => Expression::Equate(
            Box::new(parse(&expr[..idx], scope)?),
            Box::new(parse(&expr[idx + 1..], scope)?),
        ),

        Tokens::Add => Expression::Add(
            Box::new(parse(&expr[..idx], scope)?),
            Box::new(parse(&expr[idx + 1..], scope)?),
        ),
        Tokens::Subtract => Expression::Subtract(
            Box::new(parse(&expr[..idx], scope)?),
            Box::new(parse(&expr[idx + 1..], scope)?),
        ),

        Tokens::Multiply => Expression::Multiply(
            Box::new(parse(&expr[..idx], scope)?),
            Box::new(parse(&expr[idx + 1..], scope)?),
        ),

        Tokens::Divide => Expression::Divide(
            Box::new(parse(&expr[..idx], scope)?),
            Box::new(parse(&expr[idx + 1..], scope)?),
        ),

        Tokens::Power => Expression::Power(
            Box::new(parse(&expr[..idx], scope)?),
            Box::new(parse(&expr[idx + 1..], scope)?),
        ),
        _ => unreachable!(),
    })
}

fn get_lowest_precedence(expr: &[Tokens]) -> usize {
    let mut lowest = expr.first().unwrap();
    let mut index = 0;
    let mut open_brackets = 0;

    for (idx, val) in expr.iter().enumerate() {
        if matches!(val, Tokens::OpenBrackets) {
            open_brackets += 1;
        }

        if matches!(val, Tokens::CloseBrackets) {
            open_brackets -= 1;
        }

        if get_precedence_level(lowest) > get_precedence_level(val) && open_brackets == 0 {
            lowest = val;
            index = idx;
        }
    }

    index
}

fn get_precedence_level(token: &Tokens) -> u32 {
    match token {
        Tokens::Equate => 0,
        Tokens::Add | Tokens::Subtract => 1,
        Tokens::Multiply | Tokens::Divide => 2,
        Tokens::Power => 3,
        _ => 4,
    }
}
fn entire_expr_in_paren(expr: &[Tokens]) -> bool {
    if expr
        .first()
        .is_some_and(|x| !matches!(x, Tokens::OpenBrackets))
    {
        return false;
    }

    let mut open_brac = 1;

    for (idx, i) in expr[1..].iter().enumerate() {
        if matches!(i, Tokens::OpenBrackets) {
            open_brac += 1;
        }

        if matches!(i, Tokens::CloseBrackets) {
            open_brac -= 1;
        }

        if open_brac == 0 && idx != expr.len() - 2 {
            return false;
        }
    }

    true
}

fn call_function(scope: &Scope, name: &str, args: &[Vec<Tokens>]) -> f64 {
    let function = match scope.get(name).unwrap() {
        Expression::Function(i) => i,
        _ => unreachable!(),
    };

    let mut arguments = Vec::with_capacity(args.len());

    for i in args {
        arguments.push(parse(i, scope).unwrap());
    }

    function(arguments, scope)
}
