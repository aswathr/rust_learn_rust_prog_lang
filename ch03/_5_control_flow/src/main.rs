fn main() {
    println!("Hello, world!");

    // let num = 3;
    let num = 7;

    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if num {
    //     println!("num is bool");
    // } // this will not compile since `num` is not computed to a bool, unlike in some other languages where this is valid code

    if num != 0 {
        println!("num was not zero");
    }

    // multiple `if` conditions with `else if`
    if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 4, 3, or 2");
    }

    let condition = true;
    // ternary operator in Rust
    let num1 = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" }; // this is illegal code since the types in the `if` and `else` blocks are different

    println!("The value of number is: {num1}");

    // loops();
    loops1();
    loops2();
    while1();
    while2();
}

fn loops() {
    // this will run continuously till we interrupt the program with `ctrl + c`
    loop {
        println!("stuck in a loop");
    }
}

fn loops1() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // `break` will end the loop execution and the expression to the right of `break` will be returned
        }
    };

    println!("The result is {result}");
}

// toy example to illustrate labelled loops
fn loops2() {
    let mut count = 1;

    'counting_loop: loop {
        'inner_loop: loop {
            println!("value of count is {count}");

            if count % 10 == 0 {
                println!("breaking 'counting_loop");
                break 'counting_loop;
            }

            if count % 3 == 0 {
                println!("breaking 'inner_loop");
                break 'inner_loop;
            }
            count = (count * 4) + 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

fn while1() {
    let mut number = 3;

    // this condition eliminates a break statement that would otherwise be needed in a `loop` statement
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// loop through elements in a array
fn while2() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // this is not the recommened way of looping through a array since one might accidently cause a out of bounds panic
    // and also this adds a unwanted additional runtime check on the index
    while index < 5 {
        println!("the value of element is: {}", a[index]);

        index += 1;
    }

    // this is better and idiomatic
    for ele in a {
        println!("the val of the element is {ele}");
    }

    // this `for` loop works on a range
    // `rev()` reverses the `Range` that is returned by `1..4`
    for ele in (1..4).rev() {
        println!("{ele}!");
    }
    println!("LIFTOFF!!!");
}
