mod math;
use math::ArithmeticExpression as AE;

fn main() {
    let x = AE::parse("5.7 * 3.14159 + 2");
    println!("{:?}", x);
    println!("{}", x.evaluate());
}
