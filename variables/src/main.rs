fn main() {
    const THREE_HOURS_IN_SEDONDS: u32 = 60 * 60 * 3;

    let mut x = 5;
    println!("The value of x: {x}");
    x = 6;
    println!("The value of x: {x}");

    let mut myVector: Vec<u32> = Vec::new();
    let mut myOtherVector: Vec<u32> = Vec::new();

    let a = 5; 
    let a = a + 1;

    myVector.push(a);

    for &atom in &myVector {
        println!("Hi");
    }

    let mut d = "sup";
    {
        let a = a * 2;
        println!("The value of a in the inner scope is: {a}");
    }

    println!("The value of a is: {a}");
    {
        let spaces = "   ";
        let spaces = spaces.len();

        let cannot_change_type = "   ";
        let cannot_change_type = cannot_change_type.len();

        let e = 2.0;
        let i= 2.0f32;
        let ty = 47u32;

    }
    
    let trunc: u32 = 9 / 2;
    println!("{trunc}");

    let my_tuple: (u32, f64, &str) = (1, 3.888, "Some String");
    let (uwu, owo, awa) = my_tuple;

    let another_tuple: (&str,&str,&str) = ("a", "cool", "tuple");
    let (my,cool,tuple) = another_tuple;

    let first = another_tuple.0;
    let second = another_tuple.1;
    let third = another_tuple.2;

    let this_is_a_tuple: (u32, u32, u32) = (1, 2, 3);
    let (one, two, three) = this_is_a_tuple;
    let one1 = this_is_a_tuple.0;
    let two2 = this_is_a_tuple.1;
    let three3 = this_is_a_tuple.2;

    let my_array: [&str; 2] = ["First", "Second"];
    
}
