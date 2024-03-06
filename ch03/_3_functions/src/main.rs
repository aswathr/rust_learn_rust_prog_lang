fn main() {
    another_fn(5); // note that the arguments are unlabelled
    print_labeled_measurement(5, 'h');

    fn invalid_assignment() {
        // let x = (let y = 6); // The assignment statement `let y = 6` does not return a value, which is not the case in C
    }

    let y = {
        // This is a valid assignment
        let x = 6;
        x + 5 // because this expression returns a value
    };

    // a function with return type
    fn five() -> i32 {
        // 5; // with a semicolon at the end of the `5` this becomes a statement and not a return expression
        5 // this is a valid expression, that is returned
    }

    // which can be called and assigned to a variable like so
    let f = five();
}

//use snake_case while naming functions
fn another_fn(x: i32) {
    println!("x is {x}");
}

//fn with two parameters
fn print_labeled_measurement(value: i32, label: char) {
    println!("The measurement is: {value}{label}");
}
