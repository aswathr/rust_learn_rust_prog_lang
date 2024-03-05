
### Control flow

run statement(s) when a condition is true/ run repeatedly when a condition is true<br>

#### if

`if <expression>` will execute when expression is computed to `true`<br>
an optional `else` after the `if` expression will execute when the expression is computed to `false`<br>
you can chain multiple if statements logically with `else if`<br>

the ternary operator can be emulated like `let d = if true { 3 } else { 6 };`<br>

#### loop

`loop` will run the code inside the `loop` block repeatedly till a `break` is encountered or the program is manually encountered<br>
`continue` will skip the code in the rest of the loop block and repeat from the start of the `loop` block<br>
`break` can be appened with a expression that it will pass as a return value to the loop (notw that if multiple `break` statements are in the `loop` the return types have to match or be unit)<br>

`loop`s may have labels to disambiguate and the labels must begin with a single quote like `'loop_label`<br>

`while` loops are loops with conditions<br>

`for` loops operate on a collection looping through it's elements<br>
`for` loops can also work with `Range` to loop through them<br>


### Practice

Convert temperatures between Fahrenheit and Celsius.<br>
Generate the nth Fibonacci number.<br>
Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.<br>
