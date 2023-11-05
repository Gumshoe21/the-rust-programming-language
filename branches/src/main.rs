use std::cmp::Ordering;

fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false")
    }

    if number & 4 == 0 {
        println!("Number divisible by 4");
    } else if number % 3 == 0 {
        println!("Number divisible by 3");
    } else if number % 2 == 0 {
        println!("Number divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    let comparison_a = 5;
    let comparison_b = 5;

    match comparison_a.cmp(&comparison_b) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("They're equal!");
        }
    }

    match comparison_a.cmp(&comparison_b) {
        Ordering::Less => println!("Less"),
        Ordering::Greater => println!("Greater"),
        Ordering::Equal => println!("Equal"),
    }

    using_if_in_let_statement();
}

fn using_if_in_let_statement() {
    let condition = false;

    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" } // this won't compile
    println!("The value of number is {number}");
}
