### Ownership
* its how Rust manages memory
* Rust doesn't have a garbage collector
* Rust manages memory through ownership

#### Stack and heap
* Thinking of stack and heap allocations is important in systems programming
* A stack is a contiguous chunk of memory that is stores values *LIFO*
* Allocating on the heap requires the memory allocator to find a memory location that is large enough to hold the data and hold a reference to the pointer 
* Pushing onto and accessing data from a stack is generally faster since pushing and accessing are straightforward and do not have the overhead of the memory allocator finding a space/ keeping a reference/ dereferencing
* values passed into a function are pushed into a stack and popped when the function is over
* keeping track of which code is using heap data, cleaning up unused data on the heap are problems that ownership addresses

#### Ownership rules
* each value in Rust has a owner
* there can only be one owner at a time
* when the owner goes out of scope, the value will be dropped

#### String type
* To illustrate ownership, we'd need a complex data type(one that is stored in the heap) like a `String` type
* string literals (`"hello"`) are immutable. For mutable strings, Rust has the `String` type
* `let s = String::from("hello");` // `from` method in the `String` namespace
* `s.push_str(" world!");` // this is how we can mutate a string

#### Memory and allocation
* string literals are known at compile time and are immutable, so they are embedded in the binary
* `String` is mutable and so we need to allocate a amount of memory on the heap at runtime and return it when we're done with the `String` (manually or with a garbage collector)
    - The allocation is done by us using `String::from`
    - In languages with a garbage collector (GC), the GC tracks and cleans up memory
    - Without a GC, ususally it is done manually by calling `allocate` and `free` at the correct locations, correct times and exactly once each
* Rust has a different approach
    - The variable's memory is automatically returned once the variable goes out of scope
    - ```
      {
        let s = String::from("hello");
        
        // s is valid from this point forward

        // do stuff with s

      } // this scope is now over, and s is no longer valid
      ```
    - When `s` goes out of scope, Rust calls a special function automatically (`drop`) for us (where the author of `String` can put code to return the memory)
    - In C++, the pattern of deallocating resources at the end of a object's lifetime is sometimes called *Resource Acquisition Is Initialization* (RAII). (Essentially resource acquisition and resource initialization are bundled. While resource deallocation is bundled with object destruction)

#### Variables and Data Interacting with Move
* ```
   let x = 5;
   let y = x;
  ```
  * The first line binds the value of 5 in x. The second line copies the value of x and binds it to y
* ```
   let s1 = String::from("hello"); 
   let s2 = s1;
   println!("{}, world!", s1);
  ```
  * In Rust, the `String` value is not copied from `s1` to `s2`
* ![How a String looks like under the hood in Rust](assets/trpl04-01.svg "How a String looks like under the hood in Rust")
  A `String` has:
  * a pointer to the memory location
  * length of the `String`
  * capacity
* In other languages, data may be copied by a *shallow copy* or *deep copy*
  * ![Shallow copy](assets/trpl04-02.svg "Shallow copy")
  The figure above illustrates a shallow copy, i.e. only the pointer is copied
  * ![Deep copy](assets/trpl04-03.svg "Deep copy")
  In a *deep copy* (fig above), the entire data including the pointee data are copied and assigned to the new variable
  * There is also reference copy, where only a reference is copied
* ![How assigning a variable to another works in Rust](assets/trpl04-04.svg "How assigning a variable to another works in Rust")
  * In Rust, this line `let s2 = s1;` does not make either a *deep copy* or a *shallow copy* makes `s1` invalid. This in Rust terminology is called a *move*.
  * And this implies that Rust doesn't free anything when `s1` goes out of scope.
  * `println!("{}, world!", s1);` causes a error because `s1` is invalid at the location

#### Variables and Data Interacting with Clone
* If we desire a *deep copy*, *clone* is a common *method* that may be used
* `let s2 = s1.clone();` creates a deep copy of the `s1` object and stores it in `s2`

#### Stack-only data copy
* Data types like integers whose size are known at compile time and stored on stack are copied when assignment happens
* Types which inherit the `Copy` trait are copied instead of moved
* `Copy` cannot be implemented on a type, which or part of which has implemented the `Drop` trait
* Integer types, Boolean, floating point types, char, tuples (that contain types that all implements `Copy`) are all types that implement `Copy`

### Ownership and functions
* Passing a variable to a function is considered a *move*
* Like in the case of assignment to a different variable, types that have the `Copy` trait are not moved but copied

### Return values and scope
* Returning values transfer ownership to the calling function scope