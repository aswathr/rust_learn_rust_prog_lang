use std::io; // import statement; [std] -> standard library; [io] -> input/output library
use rand::Rng; // Rng is a trait and apparently we need to import it for implementers of the trait to be used in this file
use std::cmp::Ordering; // an enum that we'll use

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(0..=100); //generates a random number in the range
    println!("The secret number is {secret_number}"); // this is for debugging
    println!("Input your guess!");

    let _apples = 5; // let creates a variable; variables are immutable by default; the type of the variable is inferred here; BTW, the underscore behind apples is added because Rust does not allow unused variables, but if intentional we need to prefix it with a underscore
    let mut guess = String::new(); // `mut` makes a variable mutable; `String::new` is a function that returns a `String` instance
    // _apples = 10; // This is illegal
    // guess = str("Hello world!"); // This is legal

    io::stdin() // `stdin` is a function that handles user input; we could also use `std::io::stdin()` instead of `io::stdin()` and avoid the `use std::io` line; this returns a instance of type `std::io::Stdin`
    .read_line(&mut guess) // `read_line` gets input from the standard input; & -> reference; &mut -> mutable reference; `&mut guess` passes a mutable reference to the variable `guess`; `read_line` appends the input to the `String` argument passed
    .expect("Failed to read line"); // `read_line` returns a `Result` type (an enum whose values are `Ok` or `Err`); if the result is a error, this line will crash the program, but display the `msg`

    let guess: u32 = guess // the string we're converting into a `u32`; the new `guess` `u32` variable shadows the old `guess` `String` variable
    .trim() // removing whitespaces (like `\n`) and returns a `&str` slice
    .parse() // converts string to the type that is specifiec; returns a `Result<u32, ParseIntError>` type that is Ok if the `&str` from the previous line could be converted to a `u32`; BTW, the error type is actually defined for integer types in `core/num/mod.rs` at the line `macro_rules! from_str_radix_int_impl {`
    .expect("Please type a number!"); // this converts the `Result<>` into the underlying type `u32`

    println!("You guessed: {guess}"); // `{<variable>}` will insert the variable's value in place of `{<variable>}`

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Equal => println!("You win"),
        Ordering::Greater => println!("Too big"),
    }
}