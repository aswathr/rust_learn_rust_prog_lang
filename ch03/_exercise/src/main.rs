fn main() {
    let cl = farenheit_to_celcius(32.0);
    println!("32.0 farenheit is {:.1} celcius", cl);
    let fh = celcius_to_farenheit(0.0);
    println!("0.0 celcius is {:.1} farenheit", fh);
    let _10th_fib = nth_fibonacci(10);
    println!("the 10th fibonacci number is {_10th_fib}");
    print_christmas_carol();
}

fn farenheit_to_celcius(farenheit: f64) -> f64 {
    (farenheit - 32.0) * 5.0 / 9.0
}

fn celcius_to_farenheit(celcius: f64) -> f64 {
    (celcius * 5.0 / 9.0) + 32.0
}

fn nth_fibonacci(n: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;
    let mut n = n;

    if n == 0 {
        return first;
    } else if n == 1 {
        return second;
    } else {
        // note that the expression below is invalid. while loops cannot evaluate to a expression
        // return while n > 1 {
        //     n -= 1;
        //     let next = first + second;
        //     first = second;
        //     second = next;
        //     if n == 1 {
        //         break second;
        //     }
        // };

        return loop {
            n -= 1;
            let next = first + second;
            first = second;
            second = next;
            if n == 1 {
                break second;
            }
        };
    }
}

fn print_christmas_carol() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let lines = [
        "a partridge in a pear tree!\n",
        "Two turtle doves,\n",
        "Three French hens,\n",
        "Four calling birds,\n",
        "Five golden rings,\n",
        "Six geese a-laying,\n",
        "Seven swans a-swimming,\n",
        "Eight maids a-milking,\n",
        "Nine ladies dancing,\n",
        "Ten lords a-leaping,\n",
        "Eleven pipers piping,\n",
        "Twelve drummers drumming,\n",
    ];

    fn day_line(day_string: &str) -> String {
        format!("On the {day_string} day of Christmas,\nmy true love gave to me\n")
    }

    fn rest_of_lines(n: usize, lines: &[&str]) -> String {
        if n == 1 {
            return format!("{}", lines[0]);
        }
        return (0..n)
            .rev()
            .map(|i| {
                if i == 0 {
                    format!("And {}", lines[i])
                } else {
                    format!("{}", lines[i])
                }
            })
            .reduce(|acc, i| acc + &i) // Need to figure out how this works: how does the `&` before i converts it from a String to &str?
            .unwrap_or(String::new());
    }

    fn christmas_carol(days: &[&str], lines: &[&str]) -> String {
        (1..=12)
            .map(|i| day_line(days[i - 1]) + &rest_of_lines(i, lines) + "\n")
            .reduce(|res, i_str| res + &i_str)
            .unwrap_or(String::new())
    }
    let christmas_carol = christmas_carol(&days, &lines);
    println!("{christmas_carol}");
}
