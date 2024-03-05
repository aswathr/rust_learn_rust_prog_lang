
### Data types

#### Scalar

##### integers

integers, floating-point numbers, booleans, and characters<br>
integers are signed or unsigned ranging from 8 bit (`u8`, `i8`) to 128 bit (`u128`, `i128`)<br>
`1_000_000` is same as `1000000` but easier to read<br>

| Number literals |   Example   |
| --------------- | ----------- |
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte (u8 only)  | b'A'        |

integers can overflow; will panic and crash in debug; in release (with `--release` flag), overflow will cause two’s complement wrapping, i.e. storing `256` in a `u8` variable will get you a `0`, `257` -> `1`, and so on.<br>
check `wrapping_*`, `checked_*`, `overflowing_*` and `saturating_*` methods for different ways to handle integer operations with potential overflow<br>

##### floating point

floating points are numbers with decimal points<br>
trivia: floating point is named as such since the decimal point can float to any place value in the number<br>
all floats are signed, `f64`, `f32`<br>

##### numeric operation

add: `5 + 10`; subtract: `34.4 - 93.4`; multiply: `3.4 * 2.3`; divide: `54.3 / 3.5`; divide integers: `-5 / 2` (which equals -2); modulo (remainder): `32 % 5`<br>

##### boolean

`let f: bool = false;`<br>

##### character

`char` is the type used to represent characters in Rust<br>
`char` literats are represented with single quotes, `'s', '3', '🥲'`<br>
represented in Unicode Scalar<br>

#### Compound types

Rust has two primitive compound types: tuples, arrays

##### tuple

grouping a variety of types into a single compound type<br>
for example, a point in 3D cartesian coordinates can be represented as a tuple type: `(f64, f64, f64)`, the values can be represented as `let x: (f64, f64, f64) = (3.0, 2.3, 4.3)`<br>
accessed as `x.0`, `x.1` and `x.2`<br>
tuples without any values have a special name, `unit`<br>

##### arrays

collection of data with same type and allocated on the stack<br>
arrays have fixed number of elements once defined (`vector` could increase or decrease in size)<br>
array literals may be expressed within square brackets (`let a: [i32; 5] = [1, 2, 3, 4, 5];`, where `i32` is the type and `5` is the size of the array)<br>
`[3; 5]` is the same as `[3, 3, 3, 3, 3]`<br>
the first element of the array is `a[0]`, the second `a[1]` and so on<br>
accessing a element out of bounds will create a panic, i.e. `a[5]`, which tries to access the sixth element when there are only five elements<br>