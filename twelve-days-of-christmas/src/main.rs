fn main() {
    print_lines_in_reverse();
}

fn print_lines_in_reverse() {
    let mut current_line = 0;

    let lines = [
        "Two turtle doves ",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let days_of_christmas = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelth",
    ];
    while current_line < lines.len(){
        println!(
            "On the {} day of Christmas,",
            days_of_christmas[current_line]
        );
        println!("my true love sent to me");

        for &line in &lines[0..current_line + 1] {
            println!("{line}");
        }
        println!("A partridge in a pear tree\n");

        current_line += 1;
    }
}
