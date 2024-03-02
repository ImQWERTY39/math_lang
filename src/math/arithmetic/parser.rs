use super::ArithmeticExpression;

pub fn parse(expr: &[&str]) -> ArithmeticExpression {
    if expr.len() == 0 {
        panic!("Invalid expression");
    }

    if expr.len() == 1 {
        return ArithmeticExpression::Number(
            expr.first()
                .unwrap()
                .parse()
                .expect("This should have been a number -._-."),
        );
    }

    let idx = get_lowest_precedence_index(&expr);

    match expr[idx] {
        "+" => ArithmeticExpression::Add(
            Box::new(parse(&expr[..idx])),
            Box::new(parse(&expr[idx + 1..])),
        ),
        "-" => ArithmeticExpression::Subtract(
            Box::new(parse(&expr[..idx])),
            Box::new(parse(&expr[idx + 1..])),
        ),
        "*" => ArithmeticExpression::Multiply(
            Box::new(parse(&expr[..idx])),
            Box::new(parse(&expr[idx + 1..])),
        ),
        "/" => ArithmeticExpression::Divide(
            Box::new(parse(&expr[..idx])),
            Box::new(parse(&expr[idx + 1..])),
        ),
        _ => unreachable!(),
    }
}

fn get_lowest_precedence_index(expr: &[&str]) -> usize {
    let mut lowest = expr.first().unwrap();
    let mut index = 0;

    for (idx, val) in expr.iter().enumerate() {
        if get_precedence_level(&lowest) > get_precedence_level(val) {
            lowest = val;
            index = idx;
        }
    }

    index
}

fn get_precedence_level(token: &str) -> u32 {
    match token {
        "+" | "-" => 0,
        "*" | "/" => 1,
        _ => 2,
    }
}
