fn main() {
    let guess: u32 = "42".parse().expect("Not a number!"); // without `: u32`, the compiler will throw a error since the `parse` method needs it to infer it's return type

    //number operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    let remainder = 43 % 5;

    //tuple
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    //arrays
    let a = [1, 2, 3, 4, 5];
}
