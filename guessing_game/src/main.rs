use std::io;


fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");
    // The :: syntax in the ::new line indicates that new is 
    // an associated function of the String type. 
    // An associated function is a function that’s implemented on a type, 
    // in this case String.
    let mut guess = String::new();
    // The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
    io::stdin()
        // the line .read_line(&mut guess) calls the read_line method on the standard input handle to get input from the user. We’re also passing &mut guess as the argument to read_line to tell it what string to store the user input in. The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument. The string argument needs to be mutable so the method can change the string’s content.
// it also returns a Result value. Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant.
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
