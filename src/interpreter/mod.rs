use std::io::Write;

use crate::math::ArithmeticExpression;

pub fn mainloop() {
    let mut previous: Option<String> = None;

    loop {
        let expr = input(">>> ");

        if expr.is_empty() {
            println!();
            continue;
        }

        let expression = match previous {
            Some(i) => ArithmeticExpression::parse(expr.replace('_', &i).as_str()),
            None => ArithmeticExpression::parse(&expr),
        };

        match expression {
            Some(i) => {
                let result = i.evaluate();

                println!("{}", result);
                previous = Some(result.to_string());
            }

            None => {
                println!("Invalid expression");
                previous = None;
            }
        }
    }
}

fn input(msg: &str) -> String {
    print!("{}", msg);
    let _ = std::io::stdout().flush();

    let mut input_buffer = String::new();
    let _ = std::io::stdin().read_line(&mut input_buffer);

    input_buffer.trim().to_owned()
}
