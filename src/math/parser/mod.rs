use super::ArithmeticExpression;

pub fn parse(expr: &[&str]) -> Option<ArithmeticExpression> {
    if expr.is_empty() {
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

    if entire_expr_in_paren(expr) {
        return parse(&expr[1..expr.len() - 1]);
    }

    let idx = get_lowest_precedence_index(expr);

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
        "^" => ArithmeticExpression::Power(
            Box::new(parse(&expr[..idx])?),
            Box::new(parse(&expr[idx + 1..])?),
        ),
        _ => unreachable!(),
    })
}

fn entire_expr_in_paren(expr: &[&str]) -> bool {
    if expr.first().is_some_and(|x| *x != "(") {
        return false;
    }

    let mut open_brac = 1;

    for (idx, i) in expr[1..].iter().enumerate() {
        if *i == "(" {
            open_brac += 1;
        }

        if *i == ")" {
            open_brac -= 1;
        }

        if open_brac == 0 && idx != expr.len() - 2 {
            return false;
        }
    }

    true
}

fn get_lowest_precedence_index(expr: &[&str]) -> usize {
    let mut lowest = expr.first().unwrap();
    let mut index = 0;
    let mut open_brackets = 0;

    for (idx, val) in expr.iter().enumerate() {
        if *val == "(" {
            open_brackets += 1;
        }

        if *val == ")" {
            open_brackets -= 1;
        }

        if get_precedence_level(lowest) > get_precedence_level(val) && open_brackets == 0 {
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
        "^" => 2,
        _ => 3,
    }
}
