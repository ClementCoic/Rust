fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = five();
    println!("Five is {x}");

    let y = plus_one(x);
    println!("The value of y is: {y}")
}

fn another_function(x: i32) {
    println!("The value of x is: {x}.");
}
// fn function(signature) {corpus};
// signatures must include the type of each argument
// arguments are separated by comma

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Statements (have ;) are instructions that perform some action and do not return a value.
// e.g. variable declaration, function definition
// Expressions (do NOT have ;) evaluate to a resultant value. Let’s look at some examples.
// e.g. new scope block {}, 5+6
// Adding a semi-colomn `;` to an Expression turns it into a Statement and will not return a value.

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
