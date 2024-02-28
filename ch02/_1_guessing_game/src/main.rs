use std::io; /// import statement; [std] -> standard library; [io] -> input/output library

fn main() {
    println!("Guess the number!");
    println!("Input your guess!");

    let apples = 5; // let creates a variable; variables are immutable by default; the type of the variable is inferred here
    let mut guess = String::new(); // `mut` makes a variable mutable; `String::new` is a function that returns a `String` instance
    // apples = 10; // This is illegal
    // guess = str("Hello world!"); // This is legal

    io::stdin() // `stdin` is a function that handles user input; we could also use `std::io::stdin()` instead of `io::stdin()` and avoid the `use std::io` line; this returns a instance of type `std::io::Stdin`
    .read_line(&mut guess) // `read_line` gets input from the standard input; & -> reference; &mut -> mutable reference; `&mut guess` passes a mutable reference to the variable `guess`; `read_line` appends the input to the `String` argument passed
    .expect("Failed to read line"); // `read_line` returns a `Result` type (an enum whose values are `Ok` or `Err`); if the result is a error, this line will crash the program, but display the `msg`

    println!("You guessed: {guess}"); // `{<variable>}` will insert the variable's value in place of `{<variable>}`
}