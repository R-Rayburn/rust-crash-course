#![allow(dead_code, unused_mut, unused_variables)]

fn main() {
    // This collects any command-line arguments into a vector of Strings.
    // For example:
    //     cargo run apple banana
    // ...produces the equivalent of
    //     vec!["apple".to_string(), "banana".to_string()]
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        if arg == "sum" {
            sum();
        } else if arg == "double" {
            double();
        } else {
            count(arg);
        }
    }

    // To pass in args do `cargo run args` or `cargo run -- args`
}

fn sum() {
    let mut sum: i32 = 0;

    for i in 7..=23 {
        sum += i;
    }

    println!("The sum is {}", sum);
}

fn double() {
    let mut count: i32 = 0;
    let mut x: i32 = 1;
    while x <= 500 {
        x *= 2;
        count += 1;
    }
    println!("You can double x {} times until x is larger than 500", count)
}

fn count(arg: String) {
    let mut count: i32 = 0;
    loop {
        print!("{} ", arg);
        count += 1;
        if count >= 8 { break }
    }

    println!();
}
