use std::{collections::HashMap, io::Write};

use crate::math::{ArithmeticExpression, EvaluationResult};

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
            Some(ref i) => ArithmeticExpression::parse(expr.replace('_', &i).as_str(), &scope),
            None => ArithmeticExpression::parse(&expr, &scope),
        };

        if let None = expression {
            println!("Invalid expression");
            previous = None;
            continue;
        }

        match expression.unwrap().evaluate() {
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

    scope.insert(
        String::from("pi"),
        ArithmeticExpression::Number(3.141592653589793),
    );
    scope.insert(
        String::from('e'),
        ArithmeticExpression::Number(2.718281828459045),
    );

    scope
}

fn input(msg: &str) -> String {
    print!("{}", msg);
    let _ = std::io::stdout().flush();

    let mut input_buffer = String::new();
    let _ = std::io::stdin().read_line(&mut input_buffer);

    input_buffer
}
