fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("counter += 1; counter is now: {counter}");
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    println!("$$$$$$$$$$$$$$$$$$$$$$$$$$");
    while_loops();
    for_loops();
    reverse_range();
}

fn nested_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
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

fn another_nested_loop() {
    let mut count = 0;
    'outer_loop: loop {
        println!("count = {count}");

        let mut remaining = 10;
        'inner_loop: loop {
            println!("remaining: {remaining}");
            if remaining == 3 {
                break;
            }
            if count == 4 {
                break 'outer_loop;
            }

            remaining -= 1;
        }

        count += 1;
    }
    println!("End of all loops uwu.");
}

fn while_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_loops() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}


fn for_each_items() {
    let a = [10,20,30,40,50];

    for item in a {
        println!("the value is: {item}");
    }
}

fn reverse_range() {
    // 4 3 2 1
    for number in (1..=4).rev() {
        println!("{number}");
    }
    // 3 2 1
    println!("LIFTOFF!!!");
    for number in (1..4).rev() {
        println!("LIFTOFF!!!");
    }
}
