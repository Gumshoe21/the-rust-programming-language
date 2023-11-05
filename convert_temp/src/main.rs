fn main() {
    println!("Hello, world!");
println!("{}", convert_fahr_to_celsius(94.0));
}

fn convert_fahr_to_celsius(temp: f32) -> f32 {
    (temp - 32.0) * 5.0/9.0
}

