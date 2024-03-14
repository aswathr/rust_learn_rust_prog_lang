fn main() {
    var_and_scope();
    var_assign_stack();
    var_assign_heap();
    clone();

    let str0 = String::from("foo");
    let str1 = String::from("bar");
    take_ownership(str0); // This moves the variable into the function `take_ownership`

    // println!("str0 = {str0}"); // This line is illegal
    println!("str1 = {str1}"); // This is legal though, since the value `str1` was not moved, but a pointer to it was passed to the function `dont_take_ownership`

    let int0 = 5;
    makes_copy(int0); // The value is not moved but copied into the function
    println!("int0 = {int0}"); // This is legal since the value is not moved into the function

    return_and_ownership();
}

fn var_and_scope() {
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward
                         // do stuff with s
    } // this scope is now over, and s is no longer valid
}

fn var_assign_stack() {
    let x = 5;
    let y: i32 = x;
}

fn var_assign_heap() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("value of s1 = {s1}"); // this throws a cmopilation error `borrow of moved value`
}

fn clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // this creates a deep copy including the heap data

    println!("s1 = {}, s2 = {}", s1, s2); // this is legal
}

fn take_ownership(x: String) {
    println!("x = {x}");
}

fn makes_copy(x: i32) {
    println!("{}", x);
}

fn return_and_ownership() {
    let s1 = gives_ownership(); // `gives_ownership`'s return value gets moved here
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // `takes_and_gives_back` moves `s2`, so hereafter s2 is invalid to use. return value of `takes_and_gives_back`, is moved to s3
} // `s1` is dropped. `s2` was moved, nothing happens to `s2`. `s3` is dropped

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // `some_string` is returned and moves out to the calling function scope
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // `a_string` is returned and moves out to the calling function scope
}
