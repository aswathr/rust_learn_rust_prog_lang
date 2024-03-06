
### Control flow

* run statement(s) when a condition is true/ run repeatedly when a condition is true

#### if

* `if <expression>` will execute when expression is computed to `true`
* an optional `else` after the `if` expression will execute when the expression is computed to `false`
* you can chain multiple if statements logically with `else if`

* the ternary operator can be emulated like `let d = if true { 3 } else { 6 };`

#### loop

* `loop` will run the code inside the `loop` block repeatedly till a `break` is encountered or the program is manually encountered
* `continue` will skip the code in the rest of the loop block and repeat from the start of the `loop` block
* `break` can be appened with a expression that it will pass as a return value to the loop (notw that if multiple `break` statements are in the `loop` the return types have to match or be unit)

* `loop`s may have labels to disambiguate and the labels must begin with a single quote like `'loop_label`

* `while` loops are loops with conditions

* `for` loops operate on a collection looping through it's elements
* `for` loops can also work with `Range` to loop through them


### Practice

* Convert temperatures between Fahrenheit and Celsius.
* Generate the nth Fibonacci number.
* Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
