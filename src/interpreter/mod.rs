use crate::math::{ArithmeticExpression, EvaluationResult};
use std::f64::consts::{E, PI};
use std::{collections::HashMap, io::Write};

pub fn mainloop() {
    let mut scope = init_scope();
    let mut previous: Option<String> = None;

    loop {
        let val = input(">>> ");

        if val.is_empty() {
            println!();
            continue;
        }

        let expr = val.trim();

        if expr.is_empty() {
            continue;
        }

        let expression = match previous {
            Some(ref i) => ArithmeticExpression::parse(expr.replace('_', i).as_str()),
            None => ArithmeticExpression::parse(expr),
        };

        if expression.is_none() {
            println!("Invalid expression");
            previous = None;
            continue;
        }

        match expression.unwrap().evaluate(&scope) {
            EvaluationResult::Number(i) => {
                println!("{}", i);
                previous = Some(i.to_string());
            }
            EvaluationResult::Equality(i) => {
                println!("{}", i);
                previous = None;
            }
            EvaluationResult::Assignment(i, j) => {
                scope.insert(i, j);
                previous = None;
            }
        }
    }
}

fn init_scope() -> HashMap<String, ArithmeticExpression> {
    let mut scope = HashMap::new();

    scope.insert(String::from("pi"), ArithmeticExpression::Number(PI));
    scope.insert(String::from('e'), ArithmeticExpression::Number(E));

    scope
}

fn input(msg: &str) -> String {
    print!("{}", msg);
    let _ = std::io::stdout().flush();

    let mut input_buffer = String::new();
    let _ = std::io::stdin().read_line(&mut input_buffer);

    input_buffer
}
