fn main() {
    let mut x = 5; // without the `mut` keyword, the assignment `x = 6;` won't compile
    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // const cannot be mutable; const are computed at compile-time; convention to use uppercased characters with underscore in-between

    let y = 10;
    let y = y + 1; // shadowing `y` in the previous line

    {
        // inner scope
        let y = y * 2; // shadowing the second `y`
        println!("The value of y in the inner scope is: {y}"); // y == 22
    } // inner scope `y` shadowing ends

    println!("The value of y is: {y}"); // y == 11

    let spaces = "   ";
    let spaces = spaces.len(); // this is legal since we're creating a new variable that shadows the old variable with the same name; technically, the second `spaces` is a different variable than the first one

    let mut spaces_1 = "   ";
    // spaces_1 = spaces_1.len(); // however, this is illegal; the type of a mutable variable cannot be changed
}
