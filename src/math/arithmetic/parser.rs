use super::ArithmeticExpression;

/*
2 * (5 + (2 - (3 * 5))) / (4 / (6 + 2))

lowest precednce with no. of open brackets = 0

if left or right is (expr):
    eval(expr) and add to left or right of operator
*/

pub fn parse(expr: &[&str]) -> Option<ArithmeticExpression> {
    if expr.len() == 0 {
        return None;
    }

    if expr.len() == 1 {
        return Some(ArithmeticExpression::Number(
            expr.first()
                .unwrap()
                .parse()
                .expect("This should have been a number -._-."),
        ));
    }

    let idx = get_lowest_precedence_index(&expr);

    Some(match expr[idx] {
        "+" => ArithmeticExpression::Add(
            Box::new(parse(&expr[..idx])?),
            Box::new(parse(&expr[idx + 1..])?),
        ),
        "-" => ArithmeticExpression::Subtract(
            Box::new(parse(&expr[..idx])?),
            Box::new(parse(&expr[idx + 1..])?),
        ),
        "*" => ArithmeticExpression::Multiply(
            Box::new(parse(&expr[..idx])?),
            Box::new(parse(&expr[idx + 1..])?),
        ),
        "/" => ArithmeticExpression::Divide(
            Box::new(parse(&expr[..idx])?),
            Box::new(parse(&expr[idx + 1..])?),
        ),
        _ => unreachable!(),
    })
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
