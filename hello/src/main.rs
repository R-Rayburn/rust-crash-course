use hello::greet;
use std::collections::HashMap;
use rand::prelude::*;

fn main() {
    println!("Hello, world!");

    // Variables
    let bunnies = 2;

    // Variables w/ annotation
    let bunnies2: i32 = 4;

    // Can deconstruct variables
    let (bunnies3, carrots) = (8, 50);

    // Rust variables are immutable by default
    // This improves safety, concurrency, and speed

    // Error
    // let bunnies4 = 2;
    // bunnies4 = 4;

    // To make variables mutable
    let mut bunnies5 = 32;
    bunnies5 = 2;

    // Constant example
    // Convention is:
    //  Upper snake case
    //  Must have type
    //  Must be value available at compile time
    const WARP_FACTOR: f64 = 9.9;

    // Why use const?
    //  Good for globals
    //  They are fast

    // Scope
    let x = 5;
    {
	let y = 99;
	println!("{}, {}", x, y);
    }
    // println!("{}, {}", x, y);  // Error! y is not accessable in outer block

    // Shadowing
    let a = 5;
    {
	let a = 99;
	println!("{}", a);  // "99"
    }
    println!("{}", a);  // "5"

    // Shadow in same scope
    let mut x = 5;  // x is mutable
    let x = x;      // x is now immutable

    // Shadow by creating variable of different type
    //  useful for data manipulation that doesn't need previous representation
    let meme = "More cowbell!";
    // let meme = make_image(meme);

    // Rust guarentees Mem Safety at Comp time
    // Variables must be initialized before used
    let enigma: i32;
    // println!("{}", enigma);  // Error!


    let enigma2: i32;
    if true {
       enigma2 = 42;
    }
    // println!("enigma is {}", enigma2);  // Error! Compiler still can't guarentee that the value will be set at compile time.

    let enigma3: i32;
    if true {
        enigma3 = 42;
    } else {
        enigma3 = 7;
    }
    println!("enigma is {}", enigma3);


    // Testing methods from other files.
    // hello::greet();
    greet();

    let x_ran = rand::thread_rng().gen_range(0, 100);
    println!("{}", x_ran);
}
