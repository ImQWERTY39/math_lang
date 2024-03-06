use math2::Expression;
use std::collections::HashMap;

mod interpreter;
mod math;
mod math2;

type Scope = HashMap<String, Expression>;

fn main() {
    // interpreter::mainloop();

    let mut scope = HashMap::new();
    scope.insert(String::from("sin"), Expression::Function(1));
    scope.insert(String::from("cos"), Expression::Function(1));
    scope.insert(String::from("f"), Expression::Function(1));

    for i in [
        "2 * 3 + 4",
        "-2 * 3 + -4",
        "4.5 * 2.8",
        "2x",
        "5xyz",
        "(x + 1)(x + 2)",
        "sin(x)",
        "-cos(sin(x + 9) + 4x)",
        "f(x, -4y, z)",
        "4cos(x)",
        "(2 + 3cos(y))",
    ] {
        println!("[{}]: {:?}\n\n", i, math2::tokenizer::tokenize(i, &scope));
    }
}
