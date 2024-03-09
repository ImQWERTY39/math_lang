use crate::math::{EvaluationResult, Expression};
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
            Some(ref i) => Expression::parse(expr.replace('_', i).as_str(), &scope),
            None => Expression::parse(expr, &scope),
        };

        if expression.is_none() {
            println!("Invalid expression");
            previous = None;
            continue;
        }

        match expression.unwrap().evaluate(&scope) {
            EvaluationResult::Number(i) => {
                println!("{i}");
                previous = Some(i.to_string());
            }
            EvaluationResult::AssignNumber(i, j) => {
                scope.insert(i, Expression::Number(j));
                previous = None;
            }
        }
    }
}

fn init_scope() -> HashMap<String, Expression> {
    let mut scope = HashMap::new();

    scope.insert(String::from("pi"), Expression::Number(PI));
    scope.insert(String::from('e'), Expression::Number(E));

    /*    scope.insert(
        String::from("sin"),
        Expression::Function(Box::new(|arguments: Vec<Expression>, scope: &Scope| {
            if let EvaluationResult::Number(i) = arguments[0].evaluate(scope) {
                return i.sin();
            }

            panic!()
        })),
    );

    scope.insert(
        String::from("cos"),
        Expression::Function(Box::new(|arguments: Vec<Expression>, scope: &Scope| {
            if let EvaluationResult::Number(i) = arguments[0].evaluate(scope) {
                return i.cos();
            }

            panic!()
        })),
    );

    scope.insert(
        String::from("tan"),
        Expression::Function(Box::new(|arguments: Vec<Expression>, scope: &Scope| {
            if let EvaluationResult::Number(i) = arguments[0].evaluate(scope) {
                return i.tan();
            }

            panic!()
        })),
    );

    scope.insert(
        String::from("cosec"),
        Expression::Function(Box::new(|arguments: Vec<Expression>, scope: &Scope| {
            if let EvaluationResult::Number(i) = arguments[0].evaluate(scope) {
                return 1.0 / i.sin();
            }

            panic!()
        })),
    );

    scope.insert(
        String::from("sec"),
        Expression::Function(Box::new(|arguments: Vec<Expression>, scope: &Scope| {
            if let EvaluationResult::Number(i) = arguments[0].evaluate(scope) {
                return 1.0 / i.cos();
            }

            panic!()
        })),
    );

    scope.insert(
        String::from("cot"),
        Expression::Function(Box::new(|arguments: Vec<Expression>, scope: &Scope| {
            if let EvaluationResult::Number(i) = arguments[0].evaluate(scope) {
                return 1.0 / i.tan();
            }

            panic!()
        })),
    );

    scope.insert(
        String::from("ln"),
        Expression::Function(Box::new(|arguments: Vec<Expression>, scope: &Scope| {
            if let EvaluationResult::Number(i) = arguments[0].evaluate(scope) {
                return i.log(E);
            }

            panic!()
        })),
    );

    scope.insert(
        String::from("log"),
        Expression::Function(Box::new(|arguments: Vec<Expression>, scope: &Scope| {
            if let EvaluationResult::Number(i) = arguments[0].evaluate(scope) {
                return i.log10();
            }

            panic!()
        })),
    );

    scope.insert(
        String::from("logn"),
        Expression::Function(Box::new(|arguments: Vec<Expression>, scope: &Scope| {
            if let EvaluationResult::Number(i) = arguments[1].evaluate(scope) {
                return i.log(
                    if let EvaluationResult::Number(i) = arguments[0].evaluate(scope) {
                        i
                    } else {
                        panic!()
                    },
                );
            }

            panic!()
        })),
    );*/

    scope
}

fn input(msg: &str) -> String {
    print!("{}", msg);
    let _ = std::io::stdout().flush();

    let mut input_buffer = String::new();
    let _ = std::io::stdin().read_line(&mut input_buffer);

    input_buffer
}
