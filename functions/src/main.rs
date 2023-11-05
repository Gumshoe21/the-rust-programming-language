fn main() {
    println!("Hello, world!");
    a_function();
    a_function_with_args(24, "hi");
    a_scope_block_is_an_expression();
    this_returns_five();
    a_function_with_args(this_returns_five(), "hi");
    experimenting_with_previous_function();
    using_plus_one();
}

fn a_function() {
    println!("Another function.");
}

fn a_function_with_args(some_num: i32, some_str: &str) {
    println!("{some_num}: {some_str}");
}

fn a_scope_block_is_an_expression() {
    let y = {
        let x = 3;
        x + 1 // expressions don't have ending semicolons
    };

    println!("The value of y is: {y}");
}

fn this_returns_five() -> i32 {
    5
}

fn experimenting_with_previous_function() {
    let x = this_returns_five();
    println!("The value of x: {x}");
}

fn plus_one(x:i32)->i32 {
    x + 1
}

fn using_plus_one() {
    let x = plus_one(5);
    println!("5 plus 1 = {x}");
}
