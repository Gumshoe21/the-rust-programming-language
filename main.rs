fn main() {
    println!("Hello, world!");
    convert_fahr_to_celsius(32);
}

fn convert_fahr_to_celsius(temp: i32) -> i32 {
    (temp - 32) * 5/9
}

