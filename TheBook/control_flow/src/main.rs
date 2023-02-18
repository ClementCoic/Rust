fn main() {
    let condition = false;
    let number = if condition { 5 } else { 6 }; // each arm shall be of the same type

    println!("The value of number is: {number}");
    higher_than_five(number);
    different_than_zero(number);
    divisible_by_four_or_less(number);
    loop_divisible_by_a_or_less(number, 4);
    while_divisible_by_a_or_less(number, 4);
    counting_up2_down9();

    print_values_in_array([10, 20, 30, 40, 50]);
    countdown(number);

    let fahrenheit = 50;
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{fahrenheit} Fahrenheit is {celsius} Celsius.");

    nth_fibonacci(14);
}

fn higher_than_five(number: i32) {
    // condition must be a bool
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn different_than_zero(number: i32) {
    if number != 0 {
        println!("number was not zero.")
    }
}

fn divisible_by_four_or_less(number: i32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn loop_divisible_by_a_or_less(number: i32, a: i32) {
    let mut counter = a;

    loop {
        if number % counter == 0 {
            println!("number is divisible by {counter}");
            break;
        } else if counter == 1 {
            break;
        }
        counter -= 1;
    }
}

fn while_divisible_by_a_or_less(number: i32, a: i32) {
    let mut counter = a;

    while number % counter != 0 {
        counter -= 1;
    }

    println!("number is divisible by {counter}");
}

fn counting_up2_down9() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        'counting_down: loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break 'counting_down;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn print_values_in_array(arr: [i32; 5]) {
    // Proper iteration pattern
    for element in arr {
        println!("the value is: {element}");
    }

    // Bad pattern as index could be out of bound
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", arr[index]);

    //     index += 1;
    // }
}

fn countdown(a: i32) {
    for number in (1..a).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn fahrenheit_to_celsius(fahrenheit: u32) -> u32 {
    (fahrenheit - 32) * 5 / 9
}

fn nth_fibonacci(n: i32) {
    if n < 1 {
        println!("n shall be an integer higher than or equal to 1.");
    } else {
        let mut fib_m: i64 = 0;
        println!("Integer 1: {fib_m}");
        if n > 1 {
            let mut fib_n: i64 = 1;
            let mut memory: i64;
            for i in 1..n {
                println!("Integer {}: {fib_n}", i + 1);
                memory = fib_n;
                fib_n += fib_m;
                fib_m = memory;
            }
        }
    }
}
