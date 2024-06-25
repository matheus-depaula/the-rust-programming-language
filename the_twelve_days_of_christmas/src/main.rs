fn main() {
    let lyrics: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
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

    let numbers: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for i in 1..=12 {
        println!(
            "On the {} day of Christmas, my true love gave to me",
            numbers[i - 1]
        );

        for j in 0..i {
            if j == i - 1 {
                if i > 1 {
                    println!("And {}.", lyrics[j].to_lowercase());
                } else {
                    println!("{}.", lyrics[j]);
                }
            } else {
                println!("{},", lyrics[j]);
            }
        }

        println!("\n");
    }
}
