use std::io::stdin;
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    println!("\
        Welcome to Control Flow.
        Please pick a functionality.
        1) Convert Farenheit to Celcius.
        2) Convert Celcius to Farenheit.
        3) Return the nth fibonacci number
        4) Return a Christmas Carol with n paragraphs
    ");
    stdin().read_line(&mut input).expect("System error");
    let pick =  input.trim().parse::<i32>().expect("Menu pick should be a number");
    input.clear();
    match pick {
        1 | 2 => {
            print!("Convert value : ");
            io::stdout().flush().unwrap();
            stdin().read_line(&mut input).expect("System error");
            let val = input.trim().parse::<f64>().expect("Input should be a floating point number");
            match pick {
                1 => println!("{}", to_celcius(val)),
                2 => println!("{}", to_farenheit(val)),
                _ => println!("Unknown")
            }
        },
        3 => {
            stdin().read_line(&mut input).expect("System error");
            let n = input.trim().parse::<i64>().expect("Input should be an integer.");
            println!("{}", fibonacci(n))
        },
        4 => {
            stdin().read_line(&mut input).expect("System error");
            let n = input.trim().parse::<usize>().expect("Input should be a number between 1 and 12");
            pretty_print(christmas_carol(n), 60);
        },
        _ => println!("Unknown menu item.")
    }

}

const EXPECT_WRITE : &str = "Could not write to stdout";

fn pretty_print(text: String, line_len: i32) {
    let mut idx = 0;
    for b in text.bytes() {
        io::stdout().write(&[b]).expect(EXPECT_WRITE);
        if idx > line_len {
            if b == b'\n' {
                idx = 0
            } else if b == b' ' {
                io::stdout().write(&[b'\n']).expect(EXPECT_WRITE);
                idx = 0
            }
        }
        idx += 1
    }
    io::stdout().flush().unwrap();
}

fn to_celcius(fareinheit: f64) -> f64 {
    (fareinheit - 32.0) * 5.0/9.0
}

fn to_farenheit(celcius: f64) -> f64 {
    celcius * 9.0/5.0 + 32.0
}

fn fibonacci(n: i64) -> i64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut i = 1;
    let mut res = 0;
    let mut a = 0;
    let mut b = 1;
    while i < n {
        res = a + b;
        a = b;
        b = res;
        i += 1;
    }
    res
}

fn christmas_carol(choruses: usize) -> String {
    let mut song = String::new();
    let mut chorus = String::new();
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth", "tenth", "eleventh", "twelfth"
    ];
    let items = [
        "a partridge in a pear tree.",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five golden rings",
        "six geese a-layin'",
        "seven swans a-swimmin'",
        "eight maids a-milkin'",
        "nine lords a-leapin'",
        "ten ladies dancin'",
        "eleven pipers pipin'",
        "twelve drummers drummin'"
    ];
    for nth in 0usize..choruses {
        chorus =  capitalize(items[nth]) +
            if nth > 0 { ", " } else if nth == 1 { "and" } else { "" } + &uncapitalize(&chorus);
        song = format!("{}On the {} day of Christmas, my true love gave to me {}\n\n", song, days[nth], chorus)
    }
    song
}

fn capitalize(s: &str) -> String {
    if s.len() == 0 {
        return s.to_owned();
    }
    let capital = &s[..1];
    capital.to_uppercase() + &s[1..]
}

fn uncapitalize(s: &str) -> String {
    if s.len() == 0 {
        return s.to_owned();
    }
    let capital = &s[..1];
    capital.to_lowercase() + &s[1..]
}
