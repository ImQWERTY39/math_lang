mod interpreter;
mod math;

use math::Expression;
use std::collections::HashMap;

type Scope = HashMap<String, Expression>;

fn main() {
    interpreter::mainloop();
}
