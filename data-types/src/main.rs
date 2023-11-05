fn main() {
    let t = ([1; 2], [3; 4]);
    let (a,b) = t;
    println!("{}", a[0] + t.1[0]);

    let a =  [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    let array_tuple = ([42; 10], 2import '@/styles/tailwind.css'4;4]); 
}

