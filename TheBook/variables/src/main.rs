use std::io;

fn main() {
    // *Variables
    // let x = 5; // immutable -  trigger compile-time errors when attempting to change its value
    let mut x = 5; // mutable
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // *Constants
    const SECONDS_PER_MINUTES: u32 = 60; // 60s per min
    const MINUTES_PER_HOURS: u32 = 60; // 60min per s
    const SECONDS_PER_HOURS: u32 = MINUTES_PER_HOURS * SECONDS_PER_MINUTES; // =3600s in 1 Hr
    println!("There are {SECONDS_PER_MINUTES} seconds in 1 minute and {MINUTES_PER_HOURS} minutes per hour. Hence {SECONDS_PER_HOURS} seconds per hour.");
    const KILO: u32 = 1_000; // equivalent to 1000 and more readable
                             // *Shadowing
    let x = 5;

    let x = x + 1;

    {
        // inner scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    } // back to outer scope

    println!("The value of x is: {x}");

    // *Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("{truncated}");
    // remainder
    let remainder = 43 % 5;

    // *Boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    // *Character
    // We specify char literals with single quotes, as opposed to string literals, which use double quotes.
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // *Tuples - fixed length, different types allowed - saved on heap
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // *Arrays - fixed length, same type - saved on stack
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // let arr = [1, 2, 3, 4, 5];
    // let arr2 = [3; 5]; <=> let arr2 = [3, 3, 3, 3, 3];
    let first = arr[0];
    let second = arr[1];

    println! {"Please enter an array index."};

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];

    println!("The value of the element at index {index} is {element}");
}
